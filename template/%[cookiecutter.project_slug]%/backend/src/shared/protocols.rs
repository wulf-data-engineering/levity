use lambda_http::http::header::{ACCEPT, CONTENT_ENCODING, CONTENT_TYPE};
use lambda_http::http::StatusCode;
use lambda_http::tower::BoxError;
use lambda_http::{Body, Error, Request, Response};
use prost::Message;
use serde::de::DeserializeOwned;
use snap::raw::{Decoder, Encoder};

const APPLICATION_JSON: &str = "application/json";

const APPLICATION_X_PROTOBUF: &str = "application/x-protobuf";

// Larger protocol buffer payloads are always Snappy compressed.
const COMPRESSION_THRESHOLD: usize = 256;

/// Reads a protobuf/JSON request body into protocol type T.
pub fn read_request<T>(req: &Request) -> Result<T, Error>
where
    T: Message + Default + DeserializeOwned,
{
    match (extract_content_type(req), req.body()) {
        (APPLICATION_JSON, Body::Binary(b)) => {
            // rare case: the client sent binary with JSON content-type
            let s = std::str::from_utf8(b)
                .map_err(|e| lambda_http::Error::from(format!("Invalid UTF-8 for JSON: {}", e)))?;
            decode_json(s)
        }
        (APPLICATION_X_PROTOBUF, Body::Text(s)) => decode_binary(req, s.as_bytes()),
        (_, Body::Text(s)) => decode_json(s),
        (_, Body::Binary(b)) => decode_binary(req, b.as_ref()),
        (_, Body::Empty) => Ok(T::default()),
    }
}

fn decode_json<T>(s: &str) -> Result<T, BoxError>
where
    T: Message + Default + DeserializeOwned,
{
    serde_json::from_str(s)
        .map_err(|e| lambda_http::Error::from(format!("JSON parse error: {}", e)))
}

///
/// Write a protobuf/JSON response based on Accept header or request content-type
/// Larger protobuf payloads are Snappy compressed if the client supports it.
///
pub fn write_response<T>(resp_msg: &T, req: &Request) -> Result<Response<Body>, Error>
where
    T: Message + serde::Serialize,
{
    // Determine response content type
    let accept_header = req.headers().get(ACCEPT).and_then(|v| v.to_str().ok());

    let content_type = if let Some(accepts) = accept_header {
        match (
            accepts.find(APPLICATION_JSON),
            accepts.find(APPLICATION_X_PROTOBUF),
        ) {
            (Some(pos_json), Some(pos_protobuf)) => {
                // both present, choose the one with lower position
                if pos_json < pos_protobuf {
                    APPLICATION_JSON
                } else {
                    APPLICATION_X_PROTOBUF
                }
            }
            (Some(_), None) => APPLICATION_JSON,
            (None, Some(_)) => APPLICATION_X_PROTOBUF,
            (_, _) => extract_content_type(req),
        }
    } else {
        // fallback: mirror request content-type
        extract_content_type(req)
    };

    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, content_type);

    let body = if content_type == APPLICATION_JSON {
        let string = serde_json::to_string(resp_msg)
            .map_err(|e| lambda_http::Error::from(format!("JSON encode error: {}", e)))?;
        Body::Text(string)
    } else {
        let mut buf = Vec::with_capacity(resp_msg.encoded_len());
        resp_msg
            .encode(&mut buf)
            .map_err(|e| lambda_http::Error::from(format!("Protobuf encode error: {}", e)))?;
        if buf.len() > COMPRESSION_THRESHOLD {
            builder = builder.header(CONTENT_ENCODING, "snappy");
            let compressed = encode_snappy(&mut buf.clone())?;

            let decompressed = decode_snappy(compressed.as_slice()).unwrap();

            println!(
                "eq {} {} -> {}",
                decompressed == buf,
                buf.len(),
                compressed.len()
            );

            Body::Binary(compressed)
        } else {
            Body::Binary(buf)
        }
    };

    builder
        .body(body)
        .map_err(|e| lambda_http::Error::from(format!("Failed to build response: {}", e)))
}

fn decode_binary<T>(req: &Request, bytes: &[u8]) -> Result<T, Error>
where
    T: Message + Default + DeserializeOwned,
{
    let result = if contains_snappy(req) {
        T::decode(decode_snappy(bytes)?.as_slice())
    } else {
        T::decode(bytes)
    };

    result.map_err(|e| lambda_http::Error::from(format!("Protobuf decode error: {}", e)))
}

fn extract_content_type(req: &Request) -> &str {
    req.headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or(APPLICATION_X_PROTOBUF)
}

fn contains_snappy(req: &Request) -> bool {
    req.headers()
        .get(CONTENT_ENCODING)
        .iter()
        .any(|value| value.to_str().unwrap_or("").eq("snappy"))
}

fn decode_snappy(buf: &[u8]) -> Result<Vec<u8>, Error> {
    let mut decoder = Decoder::new();
    decoder
        .decompress_vec(buf)
        .map_err(|e| lambda_http::Error::from(format!("Snappy decompression error: {}", e)))
}

fn encode_snappy(buf: &mut Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut encoder = Encoder::new();
    encoder
        .compress_vec(buf)
        .map_err(|e| lambda_http::Error::from(format!("Snappy compression error: {}", e)))
}

use anyhow::anyhow;
use crate::{get_ssm_parameter, load_aws_config};
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::Utc;
use lambda_http::Error;
use uuid::Uuid;

const DEFAULT_TTL_SECONDS: u64 = 900;

/// Manages the registration and storage of websocket connection topics.
///
/// Topics are stored in a DynamoDB table with a TTL to allow for session-based
/// websocket connections. Each entry maps a user ID and a topic string (unique for the user).
#[derive(Clone)]
pub struct WebsocketConnections {
    db: Client,
    table_name: String,
}

impl WebsocketConnections {

    /// Creates a new instance of WebsocketConnections.
    ///
    /// Loads the DynamoDB table name from SSM parameter `/@@ cookiecutter.project_slug @@/websocket-connections-table-name`
    /// and initializes the AWS SDK client.
    pub async fn new() -> Self {
        let config = load_aws_config().await;
        
        let table_name = get_ssm_parameter(&config, "/@@ cookiecutter.project_slug @@/websocket-connections-table-name")
            .await
            .expect("Failed to get SSM parameter for websocket-connections-table-name");

        let db = Client::new(&config);

        Self { db, table_name }
    }


    /// Registers a new websocket connection topic for a user.
    ///
    /// # Arguments
    /// * `user_id` - The unique identifier of the user (e.g., Cognito sub).
    /// * `topic` - A prefix for the topic id (e.g., "process").
    /// * `id` - An optional specific ID to use as the topic suffix. If `None`,
    ///          it generates a 16-character hex suffix from the most significant
    ///          64 bits of a UUID v4.
    /// * `ttl_duration` - Optional duration for the entry TTL. Defaults to 15 minutes.
    ///
    /// # Returns
    /// The full topic id in the format `{topic}-{id}`.
    pub async fn register(
        &self,
        user_id: &str,
        topic: &str,
        id: Option<String>,
        ttl_duration: Option<std::time::Duration>,
    ) -> Result<String, Error> {
        let id = id.unwrap_or_else(|| {
            let id = Uuid::new_v4();
            let (msb, _) = id.as_u64_pair();
            format!("{:016x}", msb)
        });
        let topic_id = format!("{topic}-{id}");

        let now = Utc::now().timestamp();
        let ttl_seconds = ttl_duration.map(|d| d.as_secs()).unwrap_or(DEFAULT_TTL_SECONDS);
        let ttl = now + ttl_seconds as i64;

        self.db
            .put_item()
            .table_name(&self.table_name)
            .item("userId", AttributeValue::S(user_id.to_string()))
            .item("topicId", AttributeValue::S(topic_id.clone()))
            .item("ttl", AttributeValue::N(ttl.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to register websocket connection: {:?}", e))?;

        Ok(topic_id)
    }

    /// Sets the connectionId for a given userId and topicId.
    pub async fn set_connection_id(&self, user_id: &str, topic_id: &str, connection_id: &str) -> Result<(), Error> {
        self.db
            .update_item()
            .table_name(&self.table_name)
            .key("userId", AttributeValue::S(user_id.to_string()))
            .key("topicId", AttributeValue::S(topic_id.to_string()))
            .update_expression("SET connectionId = :c")
            .expression_attribute_values(":c", AttributeValue::S(connection_id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to update connectionId: {:?}", e))?;

        Ok(())
    }

    /// Directly upserts the connectionId for a given userId and topicId.
    /// This is useful for whitelisted topics that are not pre-registered.
    pub async fn upsert_connection_id(
        &self,
        user_id: &str,
        topic_id: &str,
        connection_id: &str,
    ) -> Result<(), Error> {
        let now = Utc::now().timestamp();
        let ttl_seconds = DEFAULT_TTL_SECONDS;
        let ttl = now + ttl_seconds as i64;

        self.db
            .put_item()
            .table_name(&self.table_name)
            .item("userId", AttributeValue::S(user_id.to_string()))
            .item("topicId", AttributeValue::S(topic_id.to_string()))
            .item("connectionId", AttributeValue::S(connection_id.to_string()))
            .item("ttl", AttributeValue::N(ttl.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to upsert connectionId: {:?}", e))?;

        Ok(())
    }

    /// Looks up a connection via the `connection-index` GSI and clears its connectionId.
    pub async fn clear_connection(&self, connection_id: &str) -> Result<(), Error> {
        let items = self.db
            .query()
            .table_name(&self.table_name)
            .index_name("connection-index")
            .key_condition_expression("connectionId = :c")
            .expression_attribute_values(":c", AttributeValue::S(connection_id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to query connection-index: {:?}", e))?;

        if let Some(item) = items.items().first() {
            let user_id = item.get("userId").ok_or_else(|| anyhow!("Missing userId"))?.as_s().map_err(|e| anyhow!("{:?}", e))?.to_string();
            let topic_id = item.get("topicId").ok_or_else(|| anyhow!("Missing topicId"))?.as_s().map_err(|e| anyhow!("{:?}", e))?.to_string();

            self.db
                .update_item()
                .table_name(&self.table_name)
                .key("userId", AttributeValue::S(user_id))
                .key("topicId", AttributeValue::S(topic_id))
                .update_expression("REMOVE connectionId")
                .send()
                .await
                .map_err(|e| anyhow!("Failed to remove connectionId: {:?}", e))?;
        }
        
        Ok(())
    }

    /// Looks up the connectionId for a given userId and topicId.
    pub async fn get_connection_id(&self, user_id: &str, topic_id: &str) -> Result<Option<String>, Error> {
        let item = self.db
            .get_item()
            .table_name(&self.table_name)
            .key("userId", AttributeValue::S(user_id.to_string()))
            .key("topicId", AttributeValue::S(topic_id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get topic: {:?}", e))?;

        if let Some(mut item) = item.item {
            let conn_id = item
                .remove("connectionId")
                .and_then(|v| v.as_s().ok().cloned());
            Ok(conn_id)
        } else {
            Ok(None)
        }
    }

    /// Sends a text message to a WebSocket connection via API Gateway Management API.
    pub async fn send_message(
        api_client: &aws_sdk_apigatewaymanagement::Client,
        connection_id: &str,
        data: &str,
    ) -> Result<(), Error> {
        let result = api_client
            .post_to_connection()
            .connection_id(connection_id)
            .data(aws_sdk_apigatewaymanagement::primitives::Blob::new(data.as_bytes()))
            .send()
            .await;

        if let Err(e) = result {
            if format!("{:?}", e).contains("GoneException") {
                return Ok(());
            }
            return Err(anyhow!("Failed to send message to connection {}: {:?}", connection_id, e).into());
        }

        Ok(())
    }

    /// Deletes / closes a WebSocket connection via API Gateway Management API.
    pub async fn delete_connection(
        api_client: &aws_sdk_apigatewaymanagement::Client,
        connection_id: &str,
    ) -> Result<(), Error> {
        let result = api_client
            .delete_connection()
            .connection_id(connection_id)
            .send()
            .await;

        if let Err(e) = result {
            if format!("{:?}", e).contains("GoneException") {
                return Ok(());
            }
            return Err(anyhow!("Failed to delete connection {}: {:?}", connection_id, e).into());
        }

        Ok(())
    }
}

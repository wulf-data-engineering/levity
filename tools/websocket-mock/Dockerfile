FROM debian:bookworm-slim

LABEL org.opencontainers.image.description="Docker image with a Rust based websocket mock server for levity."
LABEL org.opencontainers.image.source="https://github.com/wulf-data-engineering/levity/tools/websocket-mock/"

# Set the architecture from Docker Buildx
ARG TARGETARCH

# We expect the binaries to be prepared by github actions in output/$TARGETARCH
COPY output/${TARGETARCH}/websocket-mock /usr/local/bin/websocket-mock
RUN chmod +x /usr/local/bin/websocket-mock

EXPOSE 3001

ENTRYPOINT ["/usr/local/bin/websocket-mock"]

# Copyright Elasticsearch B.V. and contributors
# SPDX-License-Identifier: Apache-2.0

# To create a multi-arch image, run:
# docker buildx build --platform linux/amd64,linux/arm64 --tag elasticsearch-core-mcp-server .

FROM rust:1.88 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/elastic-mcp/Cargo.toml ./crates/elastic-mcp/

# Cache dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    mkdir -p ./crates/elastic-mcp/src/bin && \
    echo "pub fn main() {}" > ./crates/elastic-mcp/src/bin/elasticsearch-core-mcp-server.rs && \
    cargo build --release --bin elasticsearch-core-mcp-server

COPY crates ./crates/

RUN cargo build --release --bin elasticsearch-core-mcp-server

#--------------------------------------------------------------------------------------------------

FROM cgr.dev/chainguard/wolfi-base:latest

COPY --from=builder /app/target/release/elasticsearch-core-mcp-server /usr/local/bin/elasticsearch-core-mcp-server

ENTRYPOINT ["/usr/local/bin/elasticsearch-core-mcp-server"]

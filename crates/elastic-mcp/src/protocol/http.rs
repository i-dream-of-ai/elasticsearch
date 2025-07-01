// Licensed to Elasticsearch B.V. under one or more contributor
// license agreements. See the NOTICE file distributed with
// this work for additional information regarding copyright
// ownership. Elasticsearch B.V. licenses this file to you under
// the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Implementation of HTTP protocols

use crate::utils::rmcp_ext::ServerProvider;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;
use rmcp::transport::sse_server::SseServerConfig;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{SessionManager, StreamableHttpServerConfig};
use rmcp::transport::{SseServer, StreamableHttpService};
use rmcp::{RoleServer, Service};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;

/// Configuration for an HTTP MCP server
pub struct HttpServerConfig<M: SessionManager = LocalSessionManager> {
    /// TCP address to bind to
    pub bind: SocketAddr,

    /// Parent cancellation token. `serve_with_config` will return a child token
    pub ct: CancellationToken,

    /// Streamable http server option
    pub keep_alive: Option<Duration>,

    /// Streamable http server option
    pub stateful_mode: bool,

    /// Streamable http server option
    pub session_manager: Arc<M>,
}

/// An HTTP MCP server that supports both SSE and streamable HTTP.
pub struct HttpProtocol {}

impl HttpProtocol {
    pub async fn serve_with_config<S: Service<RoleServer>, M: SessionManager>(
        server_provider: impl Into<ServerProvider<S>>,
        config: HttpServerConfig<M>,
    ) -> std::io::Result<CancellationToken> {
        let server_provider = server_provider.into().0;

        let ct = config.ct.child_token();

        // Create a streamable http router
        let sh_router = {
            let sh_config = StreamableHttpServerConfig {
                sse_keep_alive: config.keep_alive,
                stateful_mode: config.stateful_mode,
            };

            let server_provider = server_provider.clone();
            // TODO: internally, new() wraps the server provider closure with an Arc. We can avoid
            // "double-Arc" by having
            let sh_service =
                StreamableHttpService::new(move || Ok(server_provider()), config.session_manager, sh_config);
            Router::new().route_service("/", sh_service)
        };

        // Create an SSE router
        let sse_router = {
            let sse_config = SseServerConfig {
                bind: config.bind,
                // SSE server will create a child cancellation token for every transport that is created
                // (see with_service() below)
                ct: ct.clone(),
                sse_keep_alive: config.keep_alive,
                sse_path: "/".to_string(),
                post_path: "/message".to_string(),
            };
            let (sse_server, sse_router) = SseServer::new(sse_config);
            let _sse_ct = sse_server.with_service(move || server_provider());

            sse_router
        };

        // Health and readiness
        // See https://kubernetes.io/docs/concepts/configuration/liveness-readiness-startup-probes/
        let health_router = {
            Router::new()
                // We may introduce a startup probe if we need to fetch/cache remote resources
                // during initialization
                // Ready: once we have the tool list we can process incoming requests
                .route("/ready", get(async || (StatusCode::OK, "Ready\n")))
                // Live: are we alive?
                .route("/live", get(async || "Alive\n"))
        };

        // Put all things together
        let main_router = Router::new()
            .nest("/sse", sse_router)
            .nest("/stream", sh_router)
            .nest("/_health", health_router)
            .with_state(());

        // Start the http server
        let listener = tokio::net::TcpListener::bind(config.bind).await?;
        let server = axum::serve(listener, main_router).with_graceful_shutdown({
            let ct = ct.clone();
            async move {
                ct.cancelled().await;
                tracing::info!("http server cancelled");
            }
        });

        // Await the server, or it will do nothing :-)
        tokio::spawn(
            async {
                let _ = server.await;
            }
            .instrument(tracing::info_span!("http-server", bind_address = %config.bind)),
        );

        Ok(ct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parts_in_extensions() {}
}

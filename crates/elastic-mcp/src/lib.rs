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

pub mod cli;
mod protocol;
mod servers;
mod utils;

use crate::cli::{HttpCommand, McpServer, McpServers, StdioCommand};
use crate::protocol::http::{HttpProtocol, HttpServerConfig};
use crate::servers::aggregate::AggregateServer;
use crate::servers::elasticsearch;
use crate::servers::proxy::ProxyServer;
use crate::utils::interpolator;
use rmcp::model::{ClientCapabilities, ClientInfo, Implementation};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::{StreamableHttpClientTransport, TokioChildProcess, stdio};
use rmcp::{RoleServer, Service, ServiceExt};
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;
use tokio::select;
use tokio_util::sync::CancellationToken;

pub async fn run_stdio(cmd: StdioCommand) -> anyhow::Result<()> {
    let (ct, handler) = crate::setup_services(&cmd.config).await?;
    let service = handler.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    select! {
        _ = service.waiting() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    ct.cancel();

    Ok(())
}

pub async fn run_http(cmd: HttpCommand) -> anyhow::Result<()> {
    let (ct, handler) = setup_services(&cmd.config).await?;
    let server_provider = move || handler.clone();
    let ct = HttpProtocol::serve_with_config(
        server_provider,
        HttpServerConfig {
            bind: cmd.address,
            ct: CancellationToken::new(),
            // streaming http:
            keep_alive: None,
            stateful_mode: false,
            session_manager: Arc::new(LocalSessionManager::default()),
        },
    )
    .await?;

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}

async fn setup_services(config: &Path) -> anyhow::Result<(CancellationToken, impl Service<RoleServer> + Clone)> {
    // Read config file and expand variables, also accepting .env files
    match dotenvy::dotenv() {
        Err(dotenvy::Error::Io(io_err)) if io_err.kind() == ErrorKind::NotFound => {}
        Err(err) => return Err(err)?,
        Ok(_) => {}
    }

    let config = std::fs::read_to_string(config)?;

    // Expand environment variables in the config file
    let config = interpolator::interpolate_from_env(config)?;

    //let jd = &mut serde_json::Deserializer::from_str(&config);
    //let config: McpServers = serde_path_to_error::deserialize(jd)?;

    // JSON5 adds comments and multiline strings (useful for ES|QL) to JSON
    let config: McpServers = match serde_json5::from_str(&config) {
        Ok(c) => c,
        Err(serde_json5::Error::Message { msg, location }) if location.is_some() => {
            let location = location.unwrap();
            let line = location.line;
            let column = location.column;
            anyhow::bail!("Failed to parse config: {msg}, at line {line} column {column}");
        }
        Err(err) => return Err(err)?,
    };

    //println!("{:#?}", config);

    let mut handlers = AggregateServer::builder();

    let ct = CancellationToken::new();

    for (name, server) in config.mcp_servers {
        tracing::info!("Adding server {name}");
        match server {
            McpServer::Elasticsearch(es) => {
                elasticsearch::ElasticsearchMcp::setup(es, &mut handlers)?;
            }

            McpServer::Stdio(stdio) => {
                let mut cmd = tokio::process::Command::new(stdio.command);
                for arg in stdio.args {
                    cmd.arg(arg);
                }
                for (k, v) in stdio.env {
                    cmd.env(k, v);
                }
                let transport = TokioChildProcess::new(cmd)?;

                let client = ().serve(transport).await?;
                handlers.push(ProxyServer::new(client, ct.clone()));
            }

            McpServer::Sse(http) => {
                // TODO: headers
                let transport = StreamableHttpClientTransport::from_uri(http.url);

                let client_info = ClientInfo {
                    protocol_version: Default::default(),
                    capabilities: ClientCapabilities::default(),
                    client_info: Implementation {
                        name: name.clone(),
                        version: "0.0.1".to_string(),
                    },
                };
                let client = client_info.serve(transport).await?;
                handlers.push(ProxyServer::new(client, ct.clone()));
            }

            McpServer::StreamableHttp(http) => {
                // TODO: headers
                let transport = StreamableHttpClientTransport::from_uri(http.url);

                let client_info = ClientInfo {
                    protocol_version: Default::default(),
                    capabilities: ClientCapabilities::default(),
                    client_info: Implementation {
                        name: name.clone(),
                        version: "0.0.1".to_string(),
                    },
                };
                let client = client_info.serve(transport).await?;
                handlers.push(ProxyServer::new(client, ct.clone()));
            }
        }
    }

    let handler = handlers.build();

    Ok((ct, handler))
}

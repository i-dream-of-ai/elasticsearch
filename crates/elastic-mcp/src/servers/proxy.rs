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

//! WORK IN PROGRESS: Proxy MCP server that forwards MCP request to another MCP client
use rmcp::model::{
    ClientNotification, ClientRequest, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo, ServerResult,
};
use rmcp::service::{NotificationContext, RequestContext, RunningService};
use rmcp::{ClientHandler, RoleClient, RoleServer, Service, ServiceError};
use tokio_util::sync::CancellationToken;

type McpResult<T> = Result<T, rmcp::Error>;

/// A server that proxies to a client instance
pub struct ProxyServer<P: ClientHandler> {
    remote: RunningService<RoleClient, P>,
}

impl<P: ClientHandler> ProxyServer<P> {
    pub fn new(remote: RunningService<RoleClient, P>, ct: CancellationToken) -> Self {
        // Cancel the child service when the parent service is cancelled
        let remote_ct = remote.cancellation_token();
        tokio::spawn(async move {
            ct.cancelled().await;
            remote_ct.cancel();
        });
        Self { remote }
    }
}

impl<P: ClientHandler> Service<RoleServer> for ProxyServer<P> {
    async fn handle_request(
        &self,
        request: ClientRequest,
        _context: RequestContext<RoleServer>,
    ) -> McpResult<ServerResult> {
        self.remote.send_request(request).await.map_err(map_err)
    }

    async fn handle_notification(
        &self,
        notification: ClientNotification,
        _context: NotificationContext<RoleServer>,
    ) -> McpResult<()> {
        self.remote.send_notification(notification).await.map_err(map_err)
    }

    fn get_info(&self) -> ServerInfo {
        // TODO
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            server_info: Implementation::default(),
            capabilities: ServerCapabilities::default(),
            instructions: None,
        }
    }
}

fn map_err(e: ServiceError) -> rmcp::Error {
    match e {
        ServiceError::McpError(re) => re,
        _ => rmcp::Error::internal_error(format!("{e}"), None),
    }
}

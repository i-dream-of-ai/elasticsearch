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

//! Aggregate MCP server.
//!
//! It accepts a number of child servers and aggregates their capabilities. This serves two purposes:
//! * split the Elasticsearch MCP server into separate code files to ease development
//! * in the future, host plugins for various application domains for Elastic products.
//!
//! Each sub-server is given an auto-incremented identifier that is added as a suffix to the identifier
//! of its tools, resources, prompts, etc. Although this idenfier then becomes visible to the outside,
//! this doesn't seem to cause any concern in discoverability.
//!
//! Another approach would be to maintain a mapping from identifier to sub-server, but it requires handling
//! potential conflicts and makes it also harder to handle dynamic changes to feature lists (e.g. dynamic
//! resource update when an index is created).
//!
use crate::utils::rmcp_ext::{DynServer, PaginatedRequest, PaginatedResult};
use futures::FutureExt;
use futures::future::BoxFuture;
use rmcp::model::*;
use rmcp::service::{NotificationContext, RequestContext};
use rmcp::{RoleServer, Service};
use std::collections::HashMap;
use std::sync::Arc;

type McpResult<T> = Result<T, rmcp::Error>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HandlerId(u32);

/// The names of each handler's tools, resources, etc. is transformed into a composite name that
/// contains the handler's id. This type provides conversion functions for that.
struct CompositeId {}

impl CompositeId {
    fn compose(handler_id: &HandlerId, item_id: &str) -> String {
        format!("{}_{}", item_id, handler_id.0)
    }

    fn split(id: &str) -> McpResult<(HandlerId, String)> {
        if let Some((item, tool)) = id.rsplit_once('_') {
            if let Ok(tool_id) = tool.parse() {
                return Ok((HandlerId(tool_id), item.to_string()));
            }
        }
        Err(rmcp::Error::resource_not_found(id.to_string(), None))
    }
}

/// Builder for [`AggregateServer`].
#[derive(Default)]
pub struct AggregateServerBuilder {
    handlers: Vec<DynServer>,
}

impl AggregateServerBuilder {
    pub fn push<T: Service<RoleServer>>(&mut self, handler: T) {
        self.handlers.push(Box::new(handler));
    }

    pub fn build(self) -> AggregateServer {
        AggregateServer::new(self.handlers)
    }
}

/// Shared data common to all clones of an AggregateHandler
#[derive(Default)]
struct AggregateSharedData {
    /// All aggregated handlers
    handlers: HashMap<HandlerId, DynServer>,
}

/// An MCP server that delegates to a number of child servers.
#[derive(Clone)]
pub struct AggregateServer {
    shared: Arc<AggregateSharedData>,
}

impl AggregateServer {
    pub fn builder() -> AggregateServerBuilder {
        AggregateServerBuilder::default()
    }

    pub fn new(handlers: Vec<DynServer>) -> Self {
        // Give an id to all handlers
        let map = handlers
            .into_iter()
            .enumerate()
            .map(|(i, h)| (HandlerId(i as u32), h))
            .collect::<HashMap<_, _>>();
        AggregateServer {
            shared: Arc::new(AggregateSharedData { handlers: map }),
        }
    }

    fn split_id(&self, id: &str) -> McpResult<(&DynServer, HandlerId, String)> {
        let (handler_id, name) = CompositeId::split(id)?;

        let Some(handler) = self.shared.handlers.get(&handler_id) else {
            return Err(rmcp::Error::resource_not_found(id.to_string(), None));
        };

        Ok((handler, handler_id, name))
    }

    fn rename_resource(&self, resource: &mut ResourceContents, id: &HandlerId) {
        match resource {
            ResourceContents::TextResourceContents { uri, .. } => {
                *uri = CompositeId::compose(id, uri);
            }
            ResourceContents::BlobResourceContents { uri, .. } => {
                *uri = CompositeId::compose(id, uri);
            }
        }
    }

    /// Generic "list all" for tools, resources, etc. from all handlers.
    #[allow(clippy::type_complexity)]
    async fn list_all<'a, 'b, U: PaginatedRequest, T: PaginatedResult>(
        &'a self,
        request: &U,
        context: &'b RequestContext<RoleServer>,

        list_items: fn(
            handler: &'a DynServer,
            request: U,
            context: &'b RequestContext<RoleServer>,
        ) -> BoxFuture<'a, McpResult<T>>,

        update_item: fn(id: &HandlerId, item: &mut T::Item),
    ) -> Result<T, rmcp::Error> {
        let handlers = &self.shared.handlers;
        // TODO: fetch concurrently on all handlers
        let mut all_items = Vec::<T::Item>::new();

        for (id, handler) in handlers {
            let mut page: Option<String> = None;

            loop {
                // Clone the request and set the pagination cursor
                let mut request = request.clone();
                request.set_page_param(page.take().map(|p| PaginatedRequestParam { cursor: Some(p) }));

                let mut response = list_items(handler, request, context).await?;

                for item in response.values().iter_mut() {
                    update_item(id, item);
                }

                all_items.append(response.values());
                if response.next_cursor().is_none() {
                    break;
                }
                page = response.next_cursor().take();
            }
        }

        Ok(T::new(all_items))
    }
}

impl Service<RoleServer> for AggregateServer {
    fn get_info(&self) -> ServerInfo {
        let mut tools = None;
        let mut prompts = None;
        let mut resources = None;
        let completions = None;
        let logging = None;
        let experimental = None;

        for handler in self.shared.handlers.values() {
            let info = Service::get_info(handler);
            if let Some(_tools) = &info.capabilities.tools {
                tools = Some(ToolsCapability::default()); // FIXME: merge list_changed
            }
            if let Some(_prompts) = &info.capabilities.prompts {
                prompts = Some(PromptsCapability::default()); // FIXME: merge list_changed
            }
            if let Some(_resources) = &info.capabilities.resources {
                resources = Some(ResourcesCapability::default()); // FIXME: merge list_changed
            }
            // FIXME: how do we merge completions?
            // FIXME: how do we merge logging? Also, only in local mode
            // FIXME: experimental ignored
        }

        let capabilities = ServerCapabilities {
            tools,
            prompts,
            resources,
            completions,
            logging,
            experimental,
        };

        // TODO: aggregate capabilities from upstream handlers
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities,
            server_info: Implementation {
                name: "Elastic-MCP".to_string(),
                version: "0.0.1".to_string(),
            },
            instructions: None,
        }
    }

    async fn handle_request(
        &self,
        request: ClientRequest,
        context: RequestContext<RoleServer>,
    ) -> McpResult<ServerResult> {
        use ClientRequest::*;

        // Experiments to access transport-level data

        // let req_meta = format!("{:?}", request.get_meta());
        //
        // let context_ext_parts = context.extensions.get::<http::request::Parts>();
        //
        // let context_meta = format!("{:?}", context.meta);
        // let context_ext_meta = format!("{:?}", context.extensions.get::<rmcp::model::Meta>());

        // let req_parts = context.extensions.get::<http::request::Parts>();
        // let req_id = TypeId::of::<http::request::Parts>();
        //
        // let resp_parts = context.extensions.get::<http::response::Parts>();
        // let resp_id = TypeId::of::<http::response::Parts>();
        //
        // //let meta = context.meta;
        // let meta_ext = context.extensions.get::<rmcp::model::Meta>();
        //
        // let req_meta = request.get_meta();
        // let req_meta2 = request.extensions().get::<rmcp::model::Meta>();

        // Get HTTP request url, method, header, etc.
        let context_ext_parts = context.extensions.get::<http::response::Parts>();

        // Get request metadata, like "progressToken"
        // https://modelcontextprotocol.io/specification/2025-03-26/basic/utilities/progress
        let context_ext_meta = context.extensions.get::<rmcp::model::Meta>();

        // Note: context.meta and request.get_meta() are empty
        let context_meta: &rmcp::model::Meta = &context.meta;

        let req_ext_meta = request.extensions().get::<rmcp::model::Meta>();
        let req_meta: &rmcp::model::Meta = request.get_meta();

        //println!("Handling request {:?}", request);

        match request {
            PingRequest(_) => {
                for handler in self.shared.handlers.values() {
                    Service::handle_request(handler, request.clone(), context.clone()).await?;
                }
                Ok(ServerResult::empty(()))
            }

            InitializeRequest(_) => {
                // TODO: aggregate capabilities from upstream handler
                // TODO: how is this related to get_info()?
                for handler in self.shared.handlers.values() {
                    Service::handle_request(handler, request.clone(), context.clone()).await?;
                }
                Ok(ServerResult::InitializeResult(Service::get_info(self)))
            }

            //----- Tools
            ListToolsRequest(request) => {
                // Collect tools from all handlers and rename them
                // FIXME: cache this expensive call?
                self.list_all(
                    &request,
                    &context,
                    |handler, request, context| {
                        async {
                            let response =
                                Service::handle_request(handler, ListToolsRequest(request), context.clone()).await?;
                            match response {
                                ServerResult::ListToolsResult(r) => Ok(r),
                                _ => Err(rmcp::Error::internal_error("Expecting ListToolsResult", None)),
                            }
                        }
                        .boxed()
                    },
                    |id, item: &mut Tool| {
                        item.name = CompositeId::compose(id, &item.name).into();
                    },
                )
                .await
                .map(ServerResult::ListToolsResult)
            }

            CallToolRequest(mut request) => {
                let (handler, id, name) = self.split_id(&request.params.name)?;
                request.params.name = name.into();

                let mut response = Service::handle_request(handler, CallToolRequest(request), context).await?;

                match response {
                    ServerResult::CallToolResult(ref mut r) => {
                        // Rewrite any resource in the response
                        for c in &mut r.content {
                            if let RawContent::Resource(rsrc) = &mut c.raw {
                                self.rename_resource(&mut rsrc.resource, &id);
                            }
                        }
                        Ok(response)
                    }
                    _ => Err(rmcp::Error::internal_error("Expecting CallToolResult", None)),
                }
            }

            //------ Resources
            ListResourcesRequest(request) => {
                // Collect resource from all handlers and rename them
                // FIXME: cache this expensive call?
                self.list_all(
                    &request,
                    &context,
                    |handler, request, context| {
                        async {
                            let response =
                                Service::handle_request(handler, ListResourcesRequest(request), context.clone())
                                    .await?;
                            match response {
                                ServerResult::ListResourcesResult(r) => Ok(r),
                                _ => Err(rmcp::Error::internal_error("Expecting ListResourcesResult", None)),
                            }
                        }
                        .boxed()
                    },
                    |id, item: &mut Resource| {
                        item.uri = CompositeId::compose(id, &item.uri);
                        item.name = CompositeId::compose(id, &item.name);
                    },
                )
                .await
                .map(ServerResult::ListResourcesResult)
            }

            ListResourceTemplatesRequest(request) => {
                // Collect resource from all handlers and rename them
                // FIXME: cache this expensive call?
                self.list_all(
                    &request,
                    &context,
                    |handler, request, context| {
                        async {
                            let response = Service::handle_request(
                                handler,
                                ListResourceTemplatesRequest(request),
                                context.clone(),
                            )
                            .await?;
                            match response {
                                ServerResult::ListResourceTemplatesResult(r) => Ok(r),
                                _ => Err(rmcp::Error::internal_error(
                                    "Expecting ListResourceTemplatesResult",
                                    None,
                                )),
                            }
                        }
                        .boxed()
                    },
                    |id, item: &mut ResourceTemplate| {
                        item.uri_template = CompositeId::compose(id, &item.uri_template);
                        item.name = CompositeId::compose(id, &item.name);
                    },
                )
                .await
                .map(ServerResult::ListResourceTemplatesResult)
            }

            ReadResourceRequest(mut request) => {
                let (handler, id, uri) = self.split_id(&request.params.uri)?;
                request.params.uri = uri;

                let mut response = Service::handle_request(handler, ReadResourceRequest(request), context).await?;
                match response {
                    ServerResult::ReadResourceResult(ref mut resp) => {
                        // Rename resources in response.
                        // Q: why can there be multiple resources in response to one resource request?
                        for resource in &mut resp.contents {
                            self.rename_resource(resource, &id);
                        }
                        Ok(response)
                    }
                    _ => Err(rmcp::Error::internal_error(
                        "Expecting ListResourceTemplatesResult",
                        None,
                    )),
                }
            }

            //----- Prompts
            ListPromptsRequest(request) => {
                // Collect prompts from all handlers and rename them
                // FIXME: cache this expensive call?
                self.list_all(
                    &request,
                    &context,
                    |handler, request, context| {
                        async {
                            let response =
                                Service::handle_request(handler, ListPromptsRequest(request), context.clone()).await?;
                            match response {
                                ServerResult::ListPromptsResult(r) => Ok(r),
                                _ => Err(rmcp::Error::internal_error("Expecting ListPromptsResult", None)),
                            }
                        }
                        .boxed()
                    },
                    |id, item: &mut Prompt| {
                        item.name = CompositeId::compose(id, &item.name);
                    },
                )
                .await
                .map(ServerResult::ListPromptsResult)
            }

            GetPromptRequest(mut request) => {
                let (handler, _id, name) = self.split_id(&request.params.name)?;
                request.params.name = name;
                Service::handle_request(handler, GetPromptRequest(request), context).await
            }

            //----- Subscriptions
            SubscribeRequest(_) => Err(rmcp::Error::method_not_found::<SubscribeRequestMethod>()),

            UnsubscribeRequest(_) => Err(rmcp::Error::method_not_found::<UnsubscribeRequestMethod>()),

            //----- Misc
            SetLevelRequest(_) => Err(rmcp::Error::method_not_found::<SetLevelRequestMethod>()),

            CompleteRequest(_) => Err(rmcp::Error::method_not_found::<CompleteRequestMethod>()),
        }
    }

    async fn handle_notification(
        &self,
        _notification: ClientNotification,
        _context: NotificationContext<RoleServer>,
    ) -> McpResult<()> {
        // Ignore for now
        // FIXME: we may want to eagerly initialize all handlers. Need to confirm with the session
        // lifecycle, as it's only worth doing if it's call only once for the lifetime of a server
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_composite_id() -> anyhow::Result<()> {
        let composite_id = CompositeId::compose(&HandlerId(1), "foo_bar");
        assert_eq!(composite_id, "foo_bar_1");

        let (handler_id, item_id) = CompositeId::split(&composite_id)?;
        assert_eq!(handler_id.0, 1);
        assert_eq!(item_id, "foo_bar");

        Ok(())
    }

    #[test]
    fn bad_composite_id() {
        let result = CompositeId::split("foo");
        assert!(result.is_err());

        let result = CompositeId::split("foo_bar");
        assert!(result.is_err());
    }
}

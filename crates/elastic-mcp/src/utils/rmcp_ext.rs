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

//! Various extensions and utilities for the Rust MCP sdk.

use rmcp::model::{
    ListPromptsRequest, ListPromptsResult, ListResourceTemplatesRequest, ListResourceTemplatesResult,
    ListResourcesRequest, ListResourcesResult, ListToolsRequest, ListToolsResult, PaginatedRequestParam, Prompt,
    Resource, ResourceTemplate, Tool,
};
use rmcp::service::DynService;
use rmcp::{RoleServer, Service};
use std::sync::Arc;

pub type DynServer = Box<dyn DynService<RoleServer>>;

//-------------------------------------------------------------------------------------------------

/// A factory to create server (`Service<RoleServer>`) instances.
pub struct ServerProvider<S: Service<RoleServer>>(pub Arc<dyn Fn() -> S + Send + Sync>);

impl<S: Service<RoleServer>> ServerProvider<S> {
    pub fn call(&self) -> S {
        self.0()
    }
}

impl<S: Service<RoleServer>, F: Fn() -> S + Send + Sync + 'static> From<F> for ServerProvider<S> {
    fn from(value: F) -> Self {
        ServerProvider(Arc::new(value))
    }
}

impl<S: Service<RoleServer>> From<Arc<dyn Fn() -> S + Send + Sync>> for ServerProvider<S> {
    fn from(value: Arc<dyn Fn() -> S + Send + Sync>) -> Self {
        ServerProvider(value)
    }
}

//-------------------------------------------------------------------------------------------------

/// Extension trait to help with pagination of the various "list" operations
pub trait PaginatedRequest: Clone {
    fn set_page_param(&mut self, params: Option<PaginatedRequestParam>);
}

pub trait PaginatedResult: Sized {
    type Item;
    fn new(items: Vec<Self::Item>) -> Self;
    fn values(&mut self) -> &mut Vec<Self::Item>;
    fn next_cursor(&mut self) -> &mut Option<String>;
}

impl PaginatedRequest for ListToolsRequest {
    fn set_page_param(&mut self, params: Option<PaginatedRequestParam>) {
        self.params = params;
    }
}

impl PaginatedResult for ListToolsResult {
    type Item = Tool;

    fn new(items: Vec<Self::Item>) -> Self {
        ListToolsResult {
            tools: items,
            next_cursor: None,
        }
    }

    fn values(&mut self) -> &mut Vec<Self::Item> {
        &mut self.tools
    }

    fn next_cursor(&mut self) -> &mut Option<String> {
        &mut self.next_cursor
    }
}

impl PaginatedRequest for ListResourcesRequest {
    fn set_page_param(&mut self, params: Option<PaginatedRequestParam>) {
        self.params = params;
    }
}

impl PaginatedResult for ListResourcesResult {
    type Item = Resource;

    fn new(items: Vec<Self::Item>) -> Self {
        ListResourcesResult {
            resources: items,
            next_cursor: None,
        }
    }

    fn values(&mut self) -> &mut Vec<Self::Item> {
        &mut self.resources
    }

    fn next_cursor(&mut self) -> &mut Option<String> {
        &mut self.next_cursor
    }
}

impl PaginatedRequest for ListResourceTemplatesRequest {
    fn set_page_param(&mut self, params: Option<PaginatedRequestParam>) {
        self.params = params;
    }
}

impl PaginatedResult for ListResourceTemplatesResult {
    type Item = ResourceTemplate;

    fn new(items: Vec<Self::Item>) -> Self {
        ListResourceTemplatesResult {
            resource_templates: items,
            next_cursor: None,
        }
    }

    fn values(&mut self) -> &mut Vec<Self::Item> {
        &mut self.resource_templates
    }

    fn next_cursor(&mut self) -> &mut Option<String> {
        &mut self.next_cursor
    }
}

impl PaginatedRequest for ListPromptsRequest {
    fn set_page_param(&mut self, params: Option<PaginatedRequestParam>) {
        self.params = params;
    }
}

impl PaginatedResult for ListPromptsResult {
    type Item = Prompt;

    fn new(items: Vec<Self::Item>) -> Self {
        ListPromptsResult {
            prompts: items,
            next_cursor: None,
        }
    }

    fn values(&mut self) -> &mut Vec<Self::Item> {
        &mut self.prompts
    }

    fn next_cursor(&mut self) -> &mut Option<String> {
        &mut self.next_cursor
    }
}

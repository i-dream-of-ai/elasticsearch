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

use rmcp::{RoleServer, Service};
use std::sync::Arc;

/// A factory to create server (`Service<RoleServer>`) instances.
pub struct ServerProvider<S: Service<RoleServer>>(pub Arc<dyn Fn() -> S + Send + Sync>);

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

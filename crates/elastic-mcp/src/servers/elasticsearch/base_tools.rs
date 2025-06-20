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

use crate::servers::elasticsearch::read_json;
use elasticsearch::cat::{CatIndicesParts, CatShardsParts};
use elasticsearch::indices::IndicesGetMappingParts;
use elasticsearch::{Elasticsearch, SearchParts};
use indexmap::IndexMap;
use rmcp::ServerHandler;
use rmcp::model::{CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp_macros::tool;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_json::{Map, Value, json};
use std::collections::HashMap;

#[derive(Clone)]
pub struct EsBaseTools {
    es_client: Elasticsearch,
}

#[tool(tool_box)]
impl EsBaseTools {
    pub fn new(es_client: Elasticsearch) -> Self {
        Self { es_client }
    }

    //---------------------------------------------------------------------------------------------
    /// Tool: list indices
    #[tool(
        description = "List all available Elasticsearch indices",
        annotations = {
            title: "List ES indices",
            readOnlyHint: true
        }
    )]
    async fn list_indices(
        &self,
        #[tool(param)]
        #[schemars(description = "Index pattern of Elasticsearch indices to list")]
        index_pattern: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        let response = self
            .es_client
            .cat()
            .indices(CatIndicesParts::Index(&[&index_pattern]))
            .h(&["index", "status", "docs.count"])
            .format("json")
            .send()
            .await;

        let response: Vec<CatIndexResponse> = read_json(response).await?;

        Ok(CallToolResult::success(vec![
            Content::text(format!("Found {} indices:", response.len())),
            Content::json(response)?,
        ]))
    }

    //---------------------------------------------------------------------------------------------
    /// Tool: get mappings for an index
    #[tool(
        description = "Get field mappings for a specific Elasticsearch index",
        annotations = {
            title: "Get ES index mappings",
            readOnlyHint: true
        })]
    async fn get_mappings(
        &self,
        #[tool(param)]
        #[schemars(description = "Name of the Elasticsearch index to get mappings for")]
        index: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        let response = self
            .es_client
            .indices()
            .get_mapping(IndicesGetMappingParts::Index(&[&index]))
            .send()
            .await;

        let response: MappingResponse = read_json(response).await?;

        // use the first mapping (we can have many if the name is a wildcard)
        let mapping = response.values().next().unwrap();

        Ok(CallToolResult::success(vec![
            Content::text(format!("Mappings for index {index}:")),
            Content::json(mapping)?,
        ]))
    }

    //---------------------------------------------------------------------------------------------
    /// Tool: search an index with the Query DSL
    ///
    /// The additional 'fields' parameter helps some LLMs that don't know about the `_source`
    /// request property to narrow down the data returned and reduce their context size
    #[tool(
        description = "Perform an Elasticsearch search with the provided query DSL.",
        annotations = {
            title: "Elasticsearch search DSL query",
            readOnlyHint: true
        }
    )]
    async fn search(
        &self,

        #[tool(param)]
        #[schemars(description = "Name of the Elasticsearch index to search")]
        index: String,

        #[tool(param)]
        #[schemars(description = "Name of the fields that need to be returned (optional)")]
        fields: Option<Vec<String>>,

        #[tool(param)]
        #[schemars(
            description = "Complete Elasticsearch query DSL object that can include query, size, from, sort, etc."
        )]
        query_body: Map<String, Value>, // note: just Value doesn't work, Claude sends a string
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut query_body = query_body;

        if let Some(fields) = fields {
            // Augment _source if it exists
            if let Some(Value::Array(values)) = query_body.get_mut("_source") {
                for field in fields.into_iter() {
                    values.push(Value::String(field))
                }
            } else {
                query_body.insert("_source".to_string(), json!(fields));
            }
        }

        let response = self
            .es_client
            .search(SearchParts::Index(&[&index]))
            .body(query_body)
            .send()
            .await;

        let response: SearchResult = read_json(response).await?;

        let mut results: Vec<Content> = Vec::new();

        // Send result stats only if it's not pure aggregation results
        if response.aggregations.is_empty() || !response.hits.hits.is_empty() {
            let total = response
                .hits
                .total
                .map(|t| t.value.to_string())
                .unwrap_or("unknown".to_string());

            results.push(Content::text(format!(
                "Total results: {}, showing {}.",
                total,
                response.hits.hits.len()
            )));
        }

        // Original prototype sent a separate content for each document, it seems to confuse some LLMs
        // for hit in &response.hits.hits {
        //     results.push(Content::json(&hit.source)?);
        // }
        if !response.hits.hits.is_empty() {
            let sources = response.hits.hits.iter().map(|hit| &hit.source).collect::<Vec<_>>();
            results.push(Content::json(&sources)?);
        }

        if !response.aggregations.is_empty() {
            results.push(Content::text("Aggregations results:"));
            results.push(Content::json(&response.aggregations)?);
        }

        Ok(CallToolResult::success(results))
    }

    //---------------------------------------------------------------------------------------------
    /// Tool: ES|QL
    #[tool(
        description = "Perform an Elasticsearch ES|QL query.",
        annotations = {
            title: "Elasticsearch ES|QL query",
            readOnlyHint: true
        })]
    async fn esql(
        &self,
        #[tool(param)]
        #[schemars(description = "Complete Elasticsearch ES|QL query.")]
        query: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        let request = EsqlQueryRequest { query };

        let response = self.es_client.esql().query().body(request).send().await;
        let response: EsqlQueryResponse = read_json(response).await?;

        // Transform response into an array of objects
        let mut objects: Vec<Value> = Vec::new();
        for row in response.values.into_iter() {
            let mut obj = Map::new();
            for (i, value) in row.into_iter().enumerate() {
                obj.insert(response.columns[i].name.clone(), value);
            }
            objects.push(Value::Object(obj));
        }

        Ok(CallToolResult::success(vec![
            Content::text("Results"),
            Content::json(objects)?,
        ]))
    }

    //---------------------------------------------------------------------------------------------
    // Tool: get shard information
    #[tool(
        description = "Get shard information for all or specific indices.",
        annotations = {
            title: "Get ES shard information",
            readOnlyHint: true
        }
    )]
    async fn get_shards(
        &self,

        #[tool(param)]
        #[schemars(description = "Optional index name to get shard information for")]
        index: Option<String>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let indices: [&str; 1];
        let parts = match &index {
            Some(index) => {
                indices = [index];
                CatShardsParts::Index(&indices)
            }
            None => CatShardsParts::None,
        };
        let response = self
            .es_client
            .cat()
            .shards(parts)
            .format("json")
            .h(&["index", "shard", "prirep", "state", "docs", "store", "node"])
            .send()
            .await;

        let response: Vec<CatShardsResponse> = read_json(response).await?;

        Ok(CallToolResult::success(vec![
            Content::text(format!("Found {} shards:", response.len())),
            Content::json(response)?,
        ]))
    }
}

#[tool(tool_box)]
impl ServerHandler for EsBaseTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Provides access to Elasticsearch".to_string()),
        }
    }
}

//-------------------------------------------------------------------------------------------------
// Type definitions for ES request/responses (the Rust client doesn't have them yet) and tool responses.

//----- Search request

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub hits: Hits,
    #[serde(default)]
    pub aggregations: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Hits {
    pub total: Option<TotalHits>,
    pub hits: Vec<Hit>,
}

#[derive(Serialize, Deserialize)]
pub struct TotalHits {
    pub value: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Hit {
    #[serde(rename = "_source")]
    pub source: Value,
}

//----- Cat responses

#[derive(Serialize, Deserialize)]
pub struct CatIndexResponse {
    pub index: String,
    pub status: String,
    #[serde(rename = "docs.count", deserialize_with = "deserialize_number_from_string")]
    pub doc_count: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CatShardsResponse {
    pub index: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub shard: usize,
    pub prirep: String,
    pub state: String,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub docs: Option<u64>,
    pub store: Option<String>,
    pub node: Option<String>,
}

//----- Index mappings

pub type MappingResponse = HashMap<String, Mappings>;

#[derive(Serialize, Deserialize)]
pub struct Mappings {
    pub mappings: Mapping,
}

#[derive(Serialize, Deserialize)]
pub struct Mapping {
    properties: HashMap<String, MappingProperty>,
}

#[derive(Serialize, Deserialize)]
pub struct MappingProperty {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub settings: HashMap<String, serde_json::Value>,
}

//----- ES|QL

#[derive(Serialize, Deserialize)]
pub struct EsqlQueryRequest {
    pub query: String,
}

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize)]
pub struct EsqlQueryResponse {
    pub is_partial: Option<bool>,
    pub columns: Vec<Column>,
    pub values: Vec<Vec<Value>>,
}

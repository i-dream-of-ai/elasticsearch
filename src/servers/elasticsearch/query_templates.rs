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

//! WORK IN PROGRESS

#![allow(dead_code)]

use rmcp::model::JsonObject;
use schemars::JsonSchema;

pub struct EsQueryTemplateTools {}

impl EsQueryTemplateTools {
    fn new() -> Self {
        Self {}

        // for (name, tool) in &self.config.tools.custom {
        //     let base = &tool.base();
        //
        //     let mut obj_val = ObjectValidation::default();
        //     for (k, v) in &base.parameters {
        //         obj_val.properties.insert(k.clone(), Schema::Object(v.clone()));
        //     }
        //     let mut obj = SchemaObject::default();
        //     obj.object = Some(Box::new(obj_val));
        //
        //     let json = match serde_json::to_value(obj).unwrap() {
        //         serde_json::Value::Object(obj) => obj,
        //         _ => panic!("unexpected schema value"),
        //     };
        //
        //
        //     list.push(rmcp::model::Tool {
        //         name: name.clone().into(),
        //         description: Some(base.description.clone().into()),
        //         input_schema: Arc::new(json),
        //         annotations: None,
        //
        //     })
        // }
        //
        // // Only keep included tools
        // if let Some(incl_excl) = &self.config.tools.incl_excl {
        //     incl_excl.filter(&mut list);
        // }
        //
        // Ok(::rmcp::model::ListToolsResult {
        //     next_cursor: None,
        //     tools: Self::tool_box().list(),
        // })
    }
}

pub fn schema_for_type<T: JsonSchema>() -> JsonObject {
    let mut settings = schemars::r#gen::SchemaSettings::default();
    settings.option_nullable = true;
    settings.option_add_null_type = false;
    settings.definitions_path = "#/components/schemas/".to_owned();
    settings.meta_schema = None;
    settings.visitors = Vec::default();
    settings.inline_subschemas = false;
    let generator = settings.into_generator();
    let schema = generator.into_root_schema_for::<T>();
    let object = serde_json::to_value(schema).expect("failed to serialize schema");
    match object {
        serde_json::Value::Object(object) => object,
        _ => panic!("unexpected schema value"),
    }
}

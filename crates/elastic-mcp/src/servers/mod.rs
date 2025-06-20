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

use serde::{Deserialize, Serialize};

pub mod aggregate;
pub mod elasticsearch;
pub mod proxy;

/// Inclusion or exclusion list.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncludeExclude {
    Include(Vec<String>),
    Exclude(Vec<String>),
}

impl IncludeExclude {
    pub fn is_included(&self, name: &str) -> bool {
        use IncludeExclude::*;
        match self {
            Include(includes) => includes.iter().map(|s| s.as_str()).any(|s| s == name),
            Exclude(excludes) => excludes.iter().map(|s| s.as_str()).all(|s| s != name),
        }
    }

    pub fn filter(&self, tools: &mut Vec<rmcp::model::Tool>) {
        tools.retain(|t| self.is_included(&t.name))
    }
}

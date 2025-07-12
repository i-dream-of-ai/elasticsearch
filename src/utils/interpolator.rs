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

//! Simple string interpolator to inject environment variables in the configuration file.
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid configuration template: {reason} at {line}:{char}")]
pub struct InterpolationError {
    pub reason: String,
    pub line: usize,
    pub char: usize,
}

pub fn interpolate_from_env(s: String) -> Result<String, InterpolationError> {
    interpolate(s, |name| std::env::var(name).ok())
}

const OPEN: &str = "${";
const OPEN_LEN: usize = OPEN.len();
const CLOSE: &str = "}";
const CLOSE_LEN: usize = CLOSE.len();

/// Simple string interpolation using the `${name}` and `${name:default_value}` syntax.
pub fn interpolate(s: String, lookup: impl Fn(&str) -> Option<String>) -> Result<String, InterpolationError> {
    if !s.contains(OPEN) {
        return Ok(s);
    }

    let mut result: String = String::new();

    for (line_no, mut line) in s.lines().enumerate() {
        if line_no > 0 {
            result.push('\n');
        }
        let mut char_no = 0;

        let err = |char_no: usize, msg: String| InterpolationError {
            reason: msg,
            line: line_no + 1, // editors (and humans) are 1-based
            char: char_no,
        };

        while let Some(pos) = line.find(OPEN) {
            // Push text before the opening brace
            result.push_str(&line[..pos]);

            char_no += pos + OPEN_LEN;
            line = &line[pos + OPEN_LEN..];

            if let Some(pos) = line.find(CLOSE) {
                let expr = &line[..pos];
                let value = if let Some((name, default)) = expr.split_once(':') {
                    lookup(name).unwrap_or(default.to_string())
                } else {
                    lookup(expr).ok_or_else(|| err(char_no, format!("env variable '{expr}' not defined")))?
                };
                result.push_str(&value);

                char_no += expr.len() + CLOSE_LEN;
                line = &line[expr.len() + CLOSE_LEN..];
            } else {
                return Err(err(char_no, "missing closing braces".to_string()));
            }
        }
        result.push_str(line);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expand(name: &str) -> Result<String, InterpolationError> {
        let lookup = |s: &str| match s {
            "foo" => Some("foo_value".to_string()),
            "bar" => Some("bar_value".to_string()),
            _ => None,
        };

        interpolate(name.to_string(), lookup)
    }

    #[test]
    fn good_extrapolation() -> anyhow::Result<()> {
        assert_eq!("012345678", expand("012345678")?);
        assert_eq!("foo_value01234", expand("${foo}01234")?);
        assert_eq!("foo_value01234\n1234bar_value", expand("${foo}01234\n1234${bar}")?);
        assert_eq!("foo_value01234bar_value", expand("${foo}01234${bar}")?);
        assert_eq!("_01_foo_value01234bar_value567", expand("_01_${foo}01234${bar}567")?);
        Ok(())
    }

    #[test]
    fn failed_extrapolation() {
        assert!(expand("${foo01234").is_err());
        assert!(expand("${foo}01234${bar").is_err());
        assert!(expand("${baz}01234").is_err());
    }
}

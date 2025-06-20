# Elasticsearch MCP Server

> [!CAUTION]
> 
> **WORK IN PROGRESS**. This is a development branch, not ready for use, things may be broken. You've been warned!

Connect to your Elasticsearch data directly from any MCP Client (like Claude Desktop) using the Model Context Protocol (MCP).

This server connects agents to your Elasticsearch data using the Model Context Protocol. It allows you to interact with your Elasticsearch indices through natural language conversations.

## Available Tools

* `list_indices`: List all available Elasticsearch indices
* `get_mappings`: Get field mappings for a specific Elasticsearch index
* `search`: Perform an Elasticsearch search with the provided query DSL
* `esql`: Perform an ES|QL query
* `get_shards`: Get shard information for all or specific indices

## Prerequisites

* An Elasticsearch instance
* Elasticsearch authentication credentials (API key or username/password)
* MCP Client (e.g. Claude Desktop)

## Installation & Setup

This branch is a development branch. This version is not packaged yet.

One-time operations:
* make sure [Rust is installed](https://www.rust-lang.org/tools/install)
* copy the `.env-example` file to `.env` and update its content according to your environment

And run
* `cargo run http` for a streamable-http server on http://localhost:8080
* `/path/to/scripts/cargo-run.sh stdio` for a stdio server (this script sets the current directory before starting `cargo run`)

## 📐 Complete Implementation Plan: writings-mcp

### Project Structure
```
writings-mcp/
├── Cargo.toml
└── src/
    ├── main.rs              # Server bootstrap with feature-based transport
    ├── handler.rs           # ServerHandler implementation
    ├── tools.rs             # Tool definitions and logic
    ├── search.rs            # Search logic (adapted from writings-api)
    └── lib.rs               # Library exports (optional)
```

### Cargo.toml
```toml
[package]
name = "writings-mcp"
version = "0.1.0"
edition = "2024"
description = "MCP server for searching Bahá'í Writings"
authors = ["Tom Grushka <tom@grushka.com>"]
license = "MIT AND Bahá'í International Community License"

[features]
default = ["stdio"]

# Stdio transport (for LLM tool integration)
stdio = ["rust-mcp-sdk/stdio"]

# HTTP transport (for web services and multi-client)
http = ["rust-mcp-sdk/hyper-server"]

# Both transports
all = ["stdio", "http"]

[dependencies]
writings = { path = "../writings", version = "0.1", features = ["embed-all", "indicium"] }
rust-mcp-sdk = { version = "0.9", default-features = false, features = ["server", "macros"] }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
log = { workspace = true }
diacritics = { workspace = true }
rapidfuzz = { workspace = true }
regex = { workspace = true }

[dev-dependencies]
tokio = { workspace = true, features = ["test-util"] }
```

### Tool Implementations (MVP: 3 Tools)

#### 1. search_writings (Primary Tool - Handles Search + Browse + Filter)
```rust
#[mcp_tool(
    name = "search_writings",
    title = "Search Bahá'í Writings",
    description = "Search writings by theme/keyword. Optional filters: type, author, kind, source. Use full_text=true for complete text (helps evaluate relevance) or full_text=false for excerpts (more efficient). CRITICAL: Always quote EXACTLY from 'text' field - never paraphrase or summarize.",
    idempotent_hint = true,
    read_only_hint = true,
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchWritingsTool {
    #[serde(default)]
    #[json_schema(description = "Search query for text content (optional - omit to browse/filter all)")]
    pub query: Option<String>,

    #[serde(default)]
    #[json_schema(description = "Filter by writing type (e.g., 'HiddenWord', 'Prayer', 'Gleaning')")]
    pub r#type: Option<String>,

    #[serde(default)]
    #[json_schema(description = "Filter by author (e.g., 'Bahaullah', 'AbdulBaha', 'TheBab')")]
    pub author: Option<String>,

    #[serde(default)]
    #[json_schema(description = "Filter by kind for HiddenWords (e.g., 'Arabic', 'Persian')")]
    pub kind: Option<String>,

    #[serde(default)]
    #[json_schema(description = "Filter by source (e.g., 'Epistle to the Son of the Wolf', 'Kitab-i-Aqdas')")]
    pub source: Option<String>,

    #[serde(default = "default_max_results")]
    #[json_schema(description = "Maximum results (default: 10, max: 100)", maximum = 100)]
    pub max_results: usize,

    #[serde(default)]
    #[json_schema(description = "Offset for pagination (default: 0)")]
    pub offset: usize,

    #[serde(default = "default_full_text")]
    #[json_schema(description = "Return complete text (true) or excerpts only (false, default). Use true when evaluating relevance for complex queries.")]
    pub full_text: bool,
}

fn default_max_results() -> usize { 10 }
fn default_full_text() -> bool { false }

// Returns: SearchResults { total, offset, limit, results: [SearchResult] }
```

**Response Format (full_text=false - excerpts only):**
```json
{
  "total": 45,
  "offset": 0,
  "limit": 10,
  "results": [{
    "score": 2847,
    "excerpt": "In the garden of thy heart plant naught but the rose of love...",
    "writings": {
      "ref_id": "607855955",
      "type": "HiddenWord",
      "author": "Bahaullah",
      "number": 3,
      "kind": "Persian",
      "url": "https://www.bahai.org/r/607855955"
    }
  }]
}
```

**Response Format (full_text=true - complete text):**
```json
{
  "total": 45,
  "offset": 0,
  "limit": 10,
  "results": [{
    "score": 2847,
    "text": "O My Servant! Abandon not for that which perisheth an everlasting dominion, and cast not away celestial sovereignty for a worldly desire. This is the river of everlasting life that hath flowed from the wellspring of the pen of the merciful; well is it with them that drink!",
    "writings": {
      "ref_id": "607855955",
      "type": "HiddenWord",
      "author": "Bahaullah",
      "number": 3,
      "kind": "Persian",
      "url": "https://www.bahai.org/r/607855955"
    }
  }]
}
```

**Usage Examples:**
- `"hidden word Arabic containing text"` → `search_writings(query="text", type="HiddenWord", kind="Arabic", full_text=false)`
- `"what are the first 3 Persian Hidden Words"` → `search_writings(type="HiddenWord", kind="Persian", max_results=3, full_text=true)`
- `"find content in Epistle to the Son of the Wolf"` → `search_writings(query="content", source="Epistle to the Son of the Wolf", full_text=true)`
- `"browse all prayers by Baha'u'lláh"` → `search_writings(type="Prayer", author="Bahaullah", full_text=false)`

#### 2. get_writing_by_ref_id
```rust
#[mcp_tool(
    name = "get_writing_by_ref_id",
    title = "Get Writing by Reference ID",
    description = "Retrieve complete exact text of a writing by its reference ID. Optionally include surrounding paragraphs for context using context_before/context_after. CRITICAL: Always quote EXACTLY - never paraphrase or summarize.",
    idempotent_hint = true,
    read_only_hint = true,
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct GetWritingByRefIdTool {
    #[json_schema(description = "Reference ID (e.g., '607855955' from https://www.bahai.org/r/607855955)")]
    pub ref_id: String,

    #[serde(default)]
    #[json_schema(description = "Number of paragraphs to include before (default: 0, max: 10)", maximum = 10)]
    pub context_before: u32,

    #[serde(default)]
    #[json_schema(description = "Number of paragraphs to include after (default: 0, max: 10)", maximum = 10)]
    pub context_after: u32,
}

// Returns: WritingResult { writing: Writings, context: optional { before, after } }
```

**Response Format (no context):**
```json
{
  "writing": {
    "ref_id": "998408191",
    "type": "HiddenWord",
    "author": "Bahaullah",
    "kind": "Persian",
    "number": 37,
    "prelude": "...and within the sanctuary of the tabernacle of God...",
    "invocation": "O My Servant!",
    "text": "O My Servant! Abandon not for that which perisheth an everlasting dominion...",
    "citations": [],
    "url": "https://www.bahai.org/r/998408191"
  }
}
```

**Response Format (with context):**
```json
{
  "writing": {
    "ref_id": "998408191",
    "type": "HiddenWord",
    "author": "Bahaullah",
    "kind": "Persian",
    "number": 37,
    "prelude": "...and within the sanctuary of the tabernacle of God...",
    "invocation": "O My Servant!",
    "text": "O My Servant! Abandon not for that which perisheth an everlasting dominion...",
    "citations": [],
    "url": "https://www.bahai.org/r/998408191"
  },
  "context": {
    "before": [
      { "ref_id": "998408190", "type": "HiddenWord", "text": "...", "url": "https://www.bahai.org/r/998408190" }
    ],
    "after": [
      { "ref_id": "998408192", "type": "HiddenWord", "text": "...", "url": "https://www.bahai.org/r/998408192" }
    ]
  }
}
```

#### 3. list_writing_metadata
```rust
#[mcp_tool(
    name = "list_writing_metadata",
    title = "List Available Writings Metadata",
    description = "Discover available writing types, counts by type, available authors, and other metadata. Use this to understand what writings are available in the database before searching.",
    idempotent_hint = true,
    read_only_hint = true,
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ListWritingMetadataTool {
    // No parameters - returns all metadata
}

// Returns: WritingMetadata { types: [...], authors: [...], counts: {...}, total: usize }
```

**Response Format:**
```json
{
  "types": [
    { "name": "HiddenWord", "count": 153 },
    { "name": "Prayer", "count": 450 },
    { "name": "Gleaning", "count": 165 },
    { "name": "Meditation", "count": 120 },
    { "name": "CDB", "count": 89 }
  ],
  "authors": [
    { "name": "Bahaullah", "count": 520 },
    { "name": "AbdulBaha", "count": 380 },
    { "name": "TheBab", "count": 77 }
  ],
  "sources": [
    "Hidden Words",
    "Prayers and Meditations",
    "Gleanings",
    "Meditations",
    "Call of the Divine Beloved"
  ],
  "kinds": {
    "HiddenWord": ["Arabic", "Persian"],
    "Prayer": ["Obligatory", "General", "Occasional", "Tablet", "Prologue"]
  },
  "total": 977
}
```

### Main.rs (Server Bootstrap)
```rust
#[cfg(feature = "stdio")]
use rust_mcp_sdk::server_runtime::stdio_transport::StdioTransport;

#[cfg(feature = "http")]
use rust_mcp_sdk::server_runtime::hyper_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let server_info = InitializeResult {
        server_info: Implementation {
            name: "writings-mcp".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            title: Some("Bahá'í Writings MCP Server".into()),
            description: Some(
                "Search and retrieve exact text from Bahá'í Writings. \
                 All responses are original text without paraphrasing or generation.".into()
            ),
            website_url: Some("https://github.com/dra11y/writings".into()),
            ..Default::default()
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools {
                list_changed: None,
            }),
            ..Default::default()
        },
        protocol_version: ProtocolVersion::V2025_11_25.into(),
        instructions: Some(
            "Search writings using search_writings(). \
             Pass query for text search, use optional filters (type, author, kind, source) to refine results. \
             Use full_text=true for complete text (helps evaluate relevance) or full_text=false for excerpts (more efficient). \
             Use get_writing_by_ref_id() to retrieve any writing. \
             Add context_before/context_after to see surrounding paragraphs. \
             Use list_writing_metadata() to discover available writings. \
             CRITICAL: Only quote exact text from these tools. Never paraphrase or generate text.".into()
        ),
        ..Default::default()
    };

    let handler = WritingsHandler::new();

    #[cfg(all(feature = "stdio", not(feature = "http")))]
    {
        let transport = StdioTransport::new(TransportOptions::default())?;
        let server = rust_mcp_sdk::server_runtime::create_server(
            McpServerOptions {
                server_details: server_info,
                transport,
                handler: handler.to_mcp_server_handler(),
                task_store: None,
            },
        )?;
        server.start().await?;
    }

    #[cfg(feature = "http")]
    {
        let server = hyper_server::create_server(
            server_info,
            handler.to_mcp_server_handler(),
            HyperServerOptions {
                host: std::env::var("HOST")
                    .unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8080),
                ..Default::default()
            },
        )?;
        server.start().await?;
    }

    Ok(())
}
```

### Handler.rs (ServerHandler Implementation)
```rust
pub struct WritingsHandler {
    _search_index: Arc<SearchIndex<String>>,
}

impl WritingsHandler {
    pub fn new() -> Self {
        let search_index = build_search_index();
        Self {
            _search_index: Arc::new(search_index),
        }
    }
}

#[async_trait]
impl ServerHandler for WritingsHandler {
    async fn handle_list_tools_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: vec![
                SearchWritingsTool::tool(),
                GetWritingByRefIdTool::tool(),
                ListWritingMetadataTool::tool(),
            ],
            meta: None,
            next_cursor: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> Result<CallToolResult, CallToolError> {
        let name = params.name.as_str();

        match name {
            "search_writings" => {
                let tool = serde_json::from_value::<SearchWritingsTool>(params.arguments)
                    .map_err(|e| CallToolError::invalid_params(&e.to_string()))?;
                tool.call()
            }
            "get_writing_by_ref_id" => {
                let tool = serde_json::from_value::<GetWritingByRefIdTool>(params.arguments)
                    .map_err(|e| CallToolError::invalid_params(&e.to_string()))?;
                tool.call()
            }
            "list_writing_metadata" => {
                ListWritingMetadataTool::default().call()
            }
            _ => Err(CallToolError::not_found(&format!("Unknown tool: {}", name))),
        }
    }
}
```

### Search.rs (Search Logic Adaptation)
```rust
use writings::{Writings, WritingsTrait as _};
use indicium::simple::{SearchIndex, SearchIndexBuilder, RapidfuzzMetric, AutocompleteType, SearchType};
use std::sync::LazyLock;

static WORD_BOUNDARY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b").unwrap());
static SENTENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^.!?]+[.!?]?\s*").unwrap());

fn build_search_index() -> SearchIndex<String> {
    let mut index: SearchIndex<String> = SearchIndexBuilder::default()
        .case_sensitive(false)
        .autocomplete_type(AutocompleteType::Global)
        .exclude_keywords(Some(
            ["thee", "thou", "thine", "hast"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .fuzzy_length(3)
        .max_autocomplete_options(9)
        .max_search_results(95)
        .rapidfuzz_metric(Some(RapidfuzzMetric::DamerauLevenshtein))
        .fuzzy_minimum_score(0.3)
        .build();

    log::info!("Loading Writings...");
    let writings = Writings::all_map();
    log::info!("Done loading Writings! Indexing...");
    writings.iter().for_each(|(ref_id, w)| {
        index.insert(ref_id, w);
    });
    log::info!("Done indexing Writings!");
    index
}

// Adapt search() function from writings-api/src/search.rs
// Key changes:
// - Add filters: type, author, kind, source
// - Support full_text parameter (true=complete text, false=excerpts only)
// - Return structured JSON with optional full text or excerpts
// - Add URL field for verification
// - Use MCP result formats
//
// For get_writing_by_ref_id: Implement context_before/context_after to retrieve surrounding paragraphs
// - Return context in structured format: { before: [...], after: [...] }
```

### Workspace Integration

Update `/Users/tom/apps/writings/Cargo.toml`:
```toml
[workspace]
members = [
    "writings",
    "writings/update",
    "writings-api",
    "writings-macros",
    "writings-mcp",  # Add this
]
resolver = "3"
```

### Build & Test Commands

```bash
# Development (stdio transport)
cargo build -p writings-mcp

# Production with HTTP
cargo build -p writings-mcp --features http

# Production with both
cargo build -p writings-mcp --features all

# Run stdio server
cargo run -p writings-mcp

# Run HTTP server
cargo run -p writings-mcp --features http

# Test
cargo test -p writings-mcp
```

### LLM Usage Patterns

**Example 1: Theme Search (with excerpts)**
```
User: "What does Bahá'u'lláh teach about unity?"
  ↓
LLM: calls search_writings(query="unity", max_results=5, include_text=true)
  ↓
MCP: 5 results with excerpts + ref_ids
  ↓
LLM: calls get_writing_by_ref_id(ref_id="607855955") for each
  ↓
MCP: Returns full exact text
  ↓
LLM: "According to Bahá'u'lláh: [exact quote 1] [exact quote 2] [exact quote 3]"
```

**Example 2: Browse by Type (with excerpts)**
```
User: "Show me the first 3 Persian Hidden Words"
  ↓
LLM: calls search_writings(type="HiddenWord", kind="Persian", max_results=3, include_text=true)
  ↓
MCP: 3 results with excerpts + metadata
  ↓
LLM: "Here are the first 3 Persian Hidden Words: [excerpts]"
```

**Example 3: Context Research (with context parameters)**
```
User: "Can you show me more context around that quote?"
  ↓
LLM: calls get_writing_by_ref_id(ref_id="607855955", context_before=3, context_after=3)
  ↓
MCP: Returns target + 3 before + 3 after paragraphs with full text
  ↓
LLM: Displays complete context with only original text
```

**Example 4: Program Assembly (references only)**
```
User: "Create a Feast program on unity with 3 prayers and 4 readings"
  ↓
LLM: calls search_writings(query="unity prayer", type="Prayer", max_results=10, include_text=false)
  ↓
MCP: Returns 10 prayer references with metadata (NO text)
  ↓
LLM: calls search_writings(query="unity", type="Prayer", author="Bahaullah", max_results=20, include_text=false)
  ↓
MCP: Returns 20 general references with metadata (NO text)
  ↓
LLM: "Here's your Feast program with references:\n\
     1. Hidden Words Persian #3 (ref_id: 607855955)\n\
     2. Prayer for Unity (ref_id: 998408456)\n\
     3. Hidden Words Arabic #12 (ref_id: 607855967)\n\
     ..."
  ↓
writings-api or user uses ref_ids to retrieve text separately
```

**Example 5: Discovery**
```
User: "What types of writings are available?"
  ↓
LLM: calls list_writing_metadata()
  ↓
MCP: Returns types, authors, counts, sources, kinds
  ↓
LLM: "Available writing types include: HiddenWords (153), Prayers (450), Gleanings (165), Meditations (120), CDB (89). Authors: Bahá'u'lláh, 'Abdu'l-Bahá, The Báb."
```

### Publishing to crates.io

```bash
# 1. Update version in writings-mcp/Cargo.toml
# 2. Test thoroughly
cargo test -p writings-mcp --features all

# 3. Publish
cargo publish -p writings-mcp

# 4. Users can depend on:
# writings-mcp = { version = "0.1", features = ["all"] }
```

## Summary

**Implementation phases:**
1. ✅ **Phase 1 (MVP)**: 3 core tools (search, get_by_ref, list_metadata) + stdio transport
2. ✅ **Phase 2**: HTTP transport support
3. ✅ **Phase 3**: Advanced features (semantic search, citations, UHJ letters)
4. ✅ **Phase 4 (Post-MVP)**: assemble_program tool + writings-api integration

**Key design decisions:**
- ✅ **Single primary search tool** (`search_writings`) with flexible parameters (MCP best practice)
  - Handles text search with optional filters (type, author, kind, source)
  - Optional parameters make it versatile
  - LLMs handle optional parameters well
- ✅ **full_text parameter** for two use cases:
  - `full_text=true`: Complete text + metadata (helps evaluate relevance)
  - `full_text=false` (default): Excerpts + metadata (more efficient, encourages full-text retrieval)
- ✅ **Context parameters in get_writing_by_ref_id** instead of separate tool:
  - Optional `context_before` and `context_after` parameters
  - Reduces tool count from 4 to 3
  - Cleaner API: one tool for retrieval + optional context
- ✅ **Discovery tool** (`list_writing_metadata`) for understanding available writings
- ✅ **Both stdio and HTTP transports** with feature flags
- ✅ **Pre-load at startup** for fast response times
- ✅ **Structured JSON + human-readable text** in responses
- ✅ **Clear LLM instructions** emphasizing exact quoting

This design enables LLMs to:
- Provide accurate, verbatim quotes from Bahá'í Writings (use `full_text=true` for relevance evaluation)
- Get references only for program assembly (use `full_text=false`)
- Retrieve writings with optional context using `context_before/context_after` parameters
- Use ref_ids that can be dereferenced by other tools (writings-api)
- Follow MCP best practices with a single flexible search tool and minimal tool count
- Prevent hallucination or paraphrasing by design

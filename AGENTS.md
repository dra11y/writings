# Agent Guidelines for writings Repository

## Build/Lint/Test Commands
- `just check` - Check code (runs `cargo check --all-targets --all-features`)
- `just test` - Run all tests (runs `cargo test --all-targets --all-features`)
- `just fix` - Auto-fix code (runs `cargo fix --allow-dirty --allow-staged`)
- `just clean` - Clean build artifacts

## Code Style Guidelines
- **Edition**: Rust 2024
- **Formatting**:
  - **ALWAYS** inline format args (e.g. `info!("inline {display} {debug:?}")`)
  - **ALWAYS** prefer guard clauses and early returns/continues/breaks to nesting if/else
- **Imports**: Use workspace dependencies when available
- **Serialization**: Serde with camelCase JSON naming via `#[serde(rename_all = "camelCase")]`
- **Error Handling**: Use wherror for custom error types, Result<T, WritingsError> alias
- **Documentation**: Comprehensive doc comments with examples
- **Tests**: Inline in modules with `#[cfg(test)]`, use `#[test]` attributes
- **Features**: Use feature flags for optional functionality (embed-all, poem, utoipa, etc.)

## Visitors Technical Overview
The writings are parsed from bahai.org HTML sources using a visitor pattern. Each writing type (HiddenWords, Gleanings, Prayers, etc.) implements the `WritingsVisitor` trait:

- **Core Trait**: `WritingsVisitor` with `visit()` method returning `VisitorAction` (VisitChildren, SkipChildren, Stop)
- **HTML Parsing**: Uses `scraper` crate with CSS selectors to navigate DOM structure
- **Class-based Matching**: Visitors match HTML elements by CSS classes (e.g., `zd hb` for prologues, `dd zd` for content)
- **State Management**: Each visitor maintains parsing state (current section, paragraph count, etc.)
- **Citation Handling**: Extracts and resolves footnotes/endnotes using `CitationText` and `resolve_citations()`
- **Text Extraction**: `ElementExt` trait provides `trimmed_text()` methods for clean text extraction with citation support
- **Validation**: Each visitor defines `EXPECTED_COUNT` and validates parsed content against known totals

## CSS Class Patterns Analysis

### Common Structural Classes (Used Across Multiple Visitors)
- `"wf"` - Footer/stop condition (Gleanings, Meditations)
- `"c q"` - Roman numeral section headers (Gleanings, Meditations)
- `"hb"` - Author/header elements (appears in Hidden Words, Prayers, CDB)
- `"zd"` - Content sections (Hidden Words: "zd hb", "dd zd hb", "dd zd")
- `"ub"` - Section headers (Prayers: "ub c l", CDB: "ub w kf")

### Document-Specific Classes

**Hidden Words:**
- `"w"` - Top invocation text
- `"zd hb"` - Prologue/epilogue sections
- `"dd zd hb"` - Prelude text (special prefaces)
- `"dd zd"` - Main hidden word content

**Gleanings & Meditations (Similar Structure):**
- `"wf"` - Footer (stop parsing)
- `"c q"` - Roman numeral section headers (I, II, III, etc.)

**Prayers (Most Complex Structure):**
- `"hb.ac"` / `"hb ac"` - Author attribution
- `"bf wf"` - Endnotes (stop condition)
- `"e"` - Title elements
- `"g c"` - Prayer kind/category
- `"ub c l"` - Section headers
- `"xc jb c kf z nb zd ub"` - Subsection headers
- `"c kf z nb zd ub"` - Teaching sections
- `"cb"` / `"z"` - Instructional text

**Call of Divine Beloved (Poetry Structure):**
- `"ic .g"` - Work titles
- `"ic .hb"` / `"ic .j"` - Work subtitles
- `"a.td"` - Paragraph numbers
- `"ub w kf"` - Invocation text
- `"span.dd"` - Poetry containers
- `"span.ce"` - Poetry lines

### Pattern Recognition for New Parsers
1. **Section Headers**: Look for `"c q"` (roman numerals) or `"ub"` variants
2. **Content Paragraphs**: Usually `"p"` elements with specific class combinations
3. **Stop Conditions**: Typically `"wf"` (footer) or `"bf wf"` (endnotes)
4. **Author Attribution**: `"hb"` variants, often combined with `"ac"`
5. **Special Text**: `"zd"` variants for prologues/epilogues, `"w"` for invocations
6. **Poetry**: Look for `"span.dd"` containers with `"span.ce"` lines

## Systematic Visitor Development Procedure

### Phase 1: HTML Analysis & Pattern Discovery
1. **Download HTML**: Fetch the target document from bahai.org and save to `writings/html/`
2. **Extract CSS Classes**: Use `grep -o 'class="[^"]*"' file.html | sort | uniq` to identify all classes
3. **Map Document Structure**: Identify key structural elements:
   - **Navigation/TOC**: Usually `nav.gc` with `ul` structure
   - **Main Content**: Look for `div.dd` containers or similar content wrappers
   - **Content Start**: Beginning of actual content (excluding preface, introductions, etc.)
   - **Section Headers**: Search for patterns like `"c q"`, `"ub c l"`, etc.
   - **Content Paragraphs**: Find `p` elements with meaningful class combinations
   - **Stop Conditions**: End of content (usually just before end notes/footers)
   - **Reference IDs**: Extract `a.sf` elements with `id` attributes for paragraph references
   - **Numbering Systems**: Document-specific numbering (roman numerals, paragraph numbers, etc.)
   - **Special Elements**: Identify citations (`sup` with `a.sf`), poetry structures, etc.

### Phase 2: Visitor Implementation
1. **Create Struct**: Define the Rust struct that will hold parsed data
2. **Implement WritingsVisitor Trait**:
   - Set `URL` and `EXPECTED_COUNT`
   - Implement `visit()` method with pattern matching
   - Add state management fields (counters, current section, etc.)
3. **CSS Class Constants**: Define `LazyLock<ClassList>` constants for each pattern
4. **State Machine Logic**: Handle document flow:
   - **Start Conditions**: When to begin parsing (after title, first section, etc.)
   - **Content Extraction**: How to extract text and metadata
   - **Transition Logic**: When to move between sections/works
   - **Stop Conditions**: When to terminate parsing

### Phase 3: Common Implementation Patterns
1. **Reference ID Extraction**: Always use `self.get_ref_id(element)` for `a.sf` elements
2. **Text Extraction**: Use `element.trimmed_text(depth, strip_newlines)` with appropriate depth
3. **Citation Handling**: For complex documents, implement citation extraction and resolution
4. **Validation**: Include `EXPECTED_COUNT` validation and test with known text samples
5. **Error Handling**: Use `panic!` for parsing errors during development, refine for production

### Phase 4: Testing & Validation
1. **Unit Tests**: Create `#[tokio::test]` with `test_visitor::<Visitor>(EXPECTED_TEXTS).await`
2. **Expected Texts**: Include 5-10 representative text samples from the document
3. **Count Validation**: Ensure `EXPECTED_COUNT` matches actual parsed items
4. **Integration**: Add to workspace and test with `just test`

### Key Insights from Existing Visitors
- **Gleanings/Meditations**: Simple structure with roman numeral sections and paragraph counting
- **Hidden Words**: Complex state management with prelude/invocation tracking and part transitions
- **Prayers**: Most complex with nested sections, author detection, and citation resolution
- **CDB**: Poetry-specific with line-by-line parsing and work title detection
- **Common Pattern**: All visitors use `VisitorAction` to control traversal flow
- **Text Processing**: `trimmed_text()` handles citation extraction and clean text normalization
- **State Management**: Each visitor maintains parsing state specific to document structure

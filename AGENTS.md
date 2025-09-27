# Agent Guidelines for writings Repository

## Build/Lint/Test Commands
- `just check` - Check code (runs `cargo check --all-targets --all-features`)
- `just test` - Run all tests (runs `cargo test --all-targets --all-features`)  
- `cargo test --package <package> --lib <test_name>` - Run single test
- `just fix` - Auto-fix code (runs `cargo fix --allow-dirty --allow-staged`)
- `just clean` - Clean build artifacts

## Code Style Guidelines
- **Edition**: Rust 2024
- **Formatting**: Standard rustfmt (no custom config)
- **Imports**: Use workspace dependencies when available
- **Naming**: snake_case for functions/vars, PascalCase for types/traits
- **Serialization**: Serde with camelCase JSON naming via `#[serde(rename_all = "camelCase")]`
- **Error Handling**: Use thiserror for custom error types, Result<T, WritingsError> alias
- **Documentation**: Comprehensive doc comments with examples
- **Tests**: Inline in modules with `#[cfg(test)]`, use `#[test]` attributes
- **Features**: Use feature flags for optional functionality (embed-all, poem, utoipa, etc.)
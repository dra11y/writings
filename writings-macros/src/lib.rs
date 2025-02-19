//! Procedural macros for the [writings](../writings) crate.

mod to_enum_schema;
mod writings_trait;

/// Derive `utoipa::ToSchema` on enum with `x-enum-descriptions` OpenAPI extension from enum strings.
#[proc_macro_derive(ToEnumSchema, attributes(schema))]
pub fn derive_to_enum_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    to_enum_schema::derive_to_enum_schema(input)
}

/// Derive the `WritingsTrait` on the `Writings` enum so we don't have to do it manually.
#[proc_macro_derive(WritingsTrait)]
pub fn derive_writings_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    writings_trait::derive_writings_trait(input)
}

use serde::{Deserialize, Serialize};

/// Whether the struct text represents an invocation ("In the name of God, the Most Glorious!"),
/// an instruction to the reader, or "normal" text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(
    feature = "utoipa",
    derive(writings_macros::ToEnumSchema),
    schema(descriptions = DocComments)
)]
pub enum ParagraphStyle {
    /// Regular Text of the Writing
    Text,

    /// Invocations (often displayed in ALL CAPS)
    Invocation,

    /// Instructions to the reader, such as those found in the Obligatory Prayers
    Instruction,
}

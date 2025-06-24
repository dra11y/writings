use serde::{Deserialize, Serialize};

/// A "footnote" or "endnote" embedded in a Text.
/// This is a second doc comment line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Citation {
    /// The reference ID from the official Bahá’í Reference Library:
    /// <https://www.bahai.org/r/`ref_id`>
    pub ref_id: String,

    /// The citation number as it appears in the text.
    pub number: u32,

    /// Relative offset (in characters) of the citation,
    /// starting from 0 at the beginning of the text it's associated with.
    pub offset: u32,

    /// The text of the footnote/endnote.
    pub text: String,
}

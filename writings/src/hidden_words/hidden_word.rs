use enumscribe::{EnumDeserialize, EnumSerialize, ScribeStaticStr, TryUnscribe, Unscribe};
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{WritingsTrait, author::Author};

/// Hidden Word
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(
    feature = "utoipa",
    derive(utoipa::ToSchema),
    schema(
        example = json!(HiddenWord {
            ref_id: "998408191".to_string(),
            kind: HiddenWordKind::Persian,
            number: Some(37),
            prelude: Some("In the first line of the Tablet it is recorded and written, and within the sanctuary of the tabernacle of God is hidden:".to_string()),
            invocation: Some("O My Servant!".to_string()),
            text: "Abandon not for that which perisheth an everlasting dominion, and cast not away celestial sovereignty for a worldly desire. This is the river of everlasting life that hath flowed from the wellspring of the pen of the merciful; well is it with them that drink!".to_string(),
        }),
    ),
)]
pub struct HiddenWord {
    /// The reference ID from the official Bahá'í Reference Library:
    /// <https://www.bahai.org/r/`ref_id`>
    pub ref_id: String,

    /// Arabic or Persian.
    pub kind: HiddenWordKind,

    /// The number of the Hidden Word. None or null for the `Prologue` (preceding Arabic) and `Epilogue` (following Persian).
    pub number: Option<u32>,

    /// Special Text preceding Persian Hidden Words #1, #20, #37, and #48.
    pub prelude: Option<String>,

    /// The first sentence (usually displayed in ALL CAPS) of each Hidden Word.
    /// (It is provided in regular sentence-case in this API.)
    pub invocation: Option<String>,

    /// The Text of the Hidden Word.
    pub text: String,
}

impl WritingsTrait for HiddenWord {
    fn ref_id(&self) -> String {
        self.ref_id.clone()
    }

    fn title(&self) -> String {
        "The Hidden Words".to_string()
    }

    fn subtitle(&self) -> Option<String> {
        Some(self.kind.title().to_string())
    }

    fn author(&self) -> crate::author::Author {
        Author::Bahaullah
    }

    fn number(&self) -> Option<u32> {
        self.number
    }

    fn paragraph_num(&self) -> u32 {
        self.number.unwrap_or(0)
    }

    fn text(&self) -> String {
        self.text.to_string()
    }
}

#[derive(
    Debug,
    Default,
    EnumSerialize,
    ScribeStaticStr,
    EnumDeserialize,
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
)]
#[enumscribe(case_insensitive)]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum HiddenWordKind {
    #[default]
    Arabic,
    Persian,
}

impl HiddenWordKind {
    pub fn language(&self) -> &str {
        self.scribe()
    }

    pub fn title(&self) -> &str {
        match self {
            HiddenWordKind::Arabic => "Part One: From the Arabic",
            HiddenWordKind::Persian => "Part Two: From the Persian",
        }
    }
}

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for HiddenWord {
    fn strings(&self) -> Vec<String> {
        [
            self.ref_id.as_str(),
            &self.kind.to_string(),
            self.prelude.as_deref().unwrap_or_default(),
            self.invocation.as_deref().unwrap_or_default(),
            &diacritics::remove_diacritics(&self.text),
        ]
        .iter()
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect()
    }
}

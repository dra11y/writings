use serde::{Deserialize, Serialize};

use crate::{WritingsTrait, author::Author};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(
    feature = "utoipa",
    derive(utoipa::ToSchema),
    schema(
        example = json!(GleaningsParagraph {
            number: 2,
            roman: "II".to_string(),
            paragraph: 1,
            ref_id: "958506325".to_string(),
            text: "The beginning of all things is the knowledge of God, and the end of all things is strict observance of whatsoever hath been sent down from the empyrean of the Divine Will that pervadeth all that is in the heavens and all that is on the earth.".to_string(),
        }),
    ),
)]
pub struct GleaningParagraph {
    /// The reference ID from the official Bahá'í Reference Library:
    /// <https://www.bahai.org/r/`ref_id`>
    pub ref_id: String,

    /// The Gleaning number in decimal format.
    pub number: u32,

    /// The Gleaning number in Roman Numeral format.
    pub roman: String,

    /// The number of the paragraph within the Gleaning, starting at 1.
    pub paragraph: u32,

    /// The actual Text of this paragraph of the Gleaning.
    pub text: String,
}

impl WritingsTrait for GleaningParagraph {
    fn ref_id(&self) -> String {
        self.ref_id.clone()
    }

    fn title(&self) -> String {
        "Gleanings from the Writings of Bahá'u'lláh".to_string()
    }

    fn subtitle(&self) -> Option<String> {
        None
    }

    fn author(&self) -> crate::author::Author {
        Author::Bahaullah
    }

    fn number(&self) -> Option<u32> {
        Some(self.number)
    }

    fn paragraph(&self) -> u32 {
        self.paragraph
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

impl GleaningParagraph {
    pub fn roman(&self) -> String {
        crate::roman::to(self.number).unwrap_or_else(|| {
            panic!("invalid Gleaning number -> Roman Numeral invalid: {self:#?}")
        })
    }
}

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for GleaningParagraph {
    fn strings(&self) -> Vec<String> {
        [
            self.ref_id.as_str(),
            self.roman.as_str(),
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

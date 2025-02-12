use serde::{Deserialize, Serialize};

use crate::{WritingsTrait, author::Author};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GleaningsParagraph {
    /// The reference ID from the official Bahá'í Reference Library:
    /// <https://www.bahai.org/r/`ref_id`>
    pub ref_id: String,

    /// The Gleaning number that appears in Roman numeral format.
    pub number: u32,

    /// The number of the paragraph within the Gleaning, starting at 1.
    pub paragraph_num: u32,

    /// The actual Text of this paragraph of the Gleaning.
    pub text: String,
}

impl WritingsTrait for GleaningsParagraph {
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

    fn paragraph_num(&self) -> u32 {
        self.paragraph_num
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

impl GleaningsParagraph {
    pub fn roman(&self) -> String {
        crate::roman::to(self.number).unwrap_or_else(|| {
            panic!("invalid Gleaning number -> Roman Numeral invalid: {self:#?}")
        })
    }
}

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for GleaningsParagraph {
    fn strings(&self) -> Vec<String> {
        [
            self.ref_id.as_str(),
            &crate::roman::to(self.number).unwrap_or_default(),
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

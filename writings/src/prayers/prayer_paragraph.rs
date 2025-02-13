use serde::{Deserialize, Serialize};

use crate::{Citation, ParagraphStyle, WritingsTrait, author::Author};

use super::{PrayerKind, prayer_source::PrayerSource};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PrayerParagraph {
    /// The reference ID from the official Bahá'í Reference Library:
    /// <https://www.bahai.org/r/`ref_id`>
    pub ref_id: String,

    /// The source, __Bahá’í Prayers__ or additional supplementary work
    /// released by the Bahá’í World Centre.
    pub source: PrayerSource,

    /// The Author of the prayer.
    pub author: Author,

    /// The “kind” or main category of prayer, if from the Bahá'í Prayers book.
    pub kind: PrayerKind,

    /// The section/subsection(s) the prayer appears in the Bahá'í Prayers book.
    pub section: Vec<String>,

    /// The number of the prayer within the section, starting at 1.
    pub number: u32,

    /// The paragraph number within the prayer, starting at 1.
    pub paragraph: u32,

    /// The “style” of the paragraph.
    pub style: ParagraphStyle,

    /// The actual Text of this paragraph of the prayer.
    pub text: String,

    /// Any [`Citation`]s (footnotes/endnotes) found within the paragraph.
    pub citations: Vec<Citation>,
}

impl WritingsTrait for PrayerParagraph {
    fn ref_id(&self) -> String {
        self.ref_id.to_string()
    }

    fn title(&self) -> String {
        self.source.title()
    }

    fn subtitle(&self) -> Option<String> {
        if self.section.is_empty() {
            return None;
        }

        let mut subtitle = String::new();

        subtitle.push_str(&self.kind.to_string());

        if let Some(section_str) = self.section.first().map(String::to_string) {
            if !subtitle.is_empty() {
                subtitle.push_str(": ");
            }
            subtitle.push_str(&section_str);
        }

        Some(subtitle)
    }

    fn author(&self) -> crate::author::Author {
        self.author
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

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for PrayerParagraph {
    fn strings(&self) -> Vec<String> {
        [
            self.ref_id.as_str(),
            &self.kind.to_string(),
            &self.section.join(" "),
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

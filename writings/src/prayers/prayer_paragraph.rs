use serde::{Deserialize, Serialize};

use crate::{Citation, ParagraphStyle, WritingsTrait, WritingsType, author::Author};

use super::{PrayerKind, prayer_source::PrayerSource};

/// A single paragraph from a [`PrayerSource`], the most well-known perhaps being <a href="https://www.bahai.org/library/authoritative-texts/prayers/bahai-prayers/" target="_blank">_Bahá’í Prayers: A Selection of Prayers Revealed by Bahá’u’lláh, the Báb, and ‘Abdu’l‑Bahá_</a>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(
    feature = "utoipa",
    derive(utoipa::ToSchema),
    schema(
        examples(
            json!(PrayerParagraph {
                ref_id: "857137774".to_string(),
                source: PrayerSource::BahaiPrayers,
                author: Author::AbdulBaha,
                kind: PrayerKind::General,
                section: vec![
                    "Teaching".to_string(),
                    "Prayers for Teaching from the Tablets of the Divine Plan".to_string(),
                    "Revealed to the Bahá’ís of the Western States".to_string()
                ],
                number: 168,
                paragraph: 3,
                style: ParagraphStyle::Text,
                text: "O Lord! I am single, alone and lowly. For me there is no support save Thee, no helper except Thee and no sustainer beside Thee. Confirm me in Thy service, assist me with the cohorts of Thy angels, make me victorious in the promotion of Thy Word and suffer me to speak out Thy wisdom amongst Thy creatures. Verily, Thou art the helper of the weak and the defender of the little ones, and verily Thou art the Powerful, the Mighty and the Unconstrained.".to_string(),
                citations: vec![],
            }),
            json!(PrayerParagraph {
                ref_id: "186814289".to_string(),
                source: PrayerSource::BahaiPrayers,
                author: Author::AbdulBaha,
                kind: PrayerKind::General,
                section: vec![
                    "Teaching".to_string(),
                    "Prayers for Teaching from the Tablets of the Divine Plan".to_string(),
                    "Revealed to the Bahá’ís of the Western States".to_string()
                ],
                number: 168,
                paragraph: 2,
                style: ParagraphStyle::Text,
                text: "O God! O God! This is a broken-winged bird and his flight is very slow—assist him so that he may fly toward the apex of prosperity and salvation, wing his way with the utmost joy and happiness throughout the illimitable space, raise his melody in Thy Supreme Name in all the regions, exhilarate the ears with this call, and brighten the eyes by beholding the signs of guidance.".to_string(),
                citations: vec![],
            }),
        ),
    ),
)]
pub struct PrayerParagraph {
    /// The reference ID from the official Bahá’í Reference Library:
    /// <https://www.bahai.org/r/`ref_id`>
    pub ref_id: String,

    /// The source, __Bahá’í Prayers__ or additional supplementary work
    /// released by the Bahá’í World Centre.
    pub source: PrayerSource,

    /// The Author of the prayer.
    pub author: Author,

    /// The “kind” or main category of prayer, if from the Bahá’í Prayers book.
    pub kind: PrayerKind,

    /// The section/subsection(s) the prayer appears in the Bahá’í Prayers book.
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
    fn ty(&self) -> WritingsType {
        WritingsType::Prayer
    }

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

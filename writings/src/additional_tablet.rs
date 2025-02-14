use serde::{Deserialize, Serialize};

use crate::{WritingsTrait, author::Author};

/// TODO: Represent a paragraph from a [`TabletSource`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TabletParagraph {
    pub source: TabletSource,
    pub ref_id: String,
    pub number: Option<u32>,
    pub paragraph: u32,
    pub text: String,
}

impl WritingsTrait for TabletParagraph {
    fn ref_id(&self) -> String {
        self.ref_id.clone()
    }

    fn title(&self) -> String {
        self.source.title()
    }

    fn subtitle(&self) -> Option<String> {
        None
    }

    fn author(&self) -> Author {
        self.source.author()
    }

    fn number(&self) -> Option<u32> {
        self.number
    }

    fn paragraph(&self) -> u32 {
        self.paragraph
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for TabletParagraph {
    fn strings(&self) -> Vec<String> {
        [
            self.ref_id.as_str(),
            &self.source.to_string(),
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

/// TODO: A work representing additional revealed Tablets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TabletSource {
    /// from <a target="_blank" href="https://www.bahai.org/library/authoritative-texts/bahaullah/additional-tablets-extracts-from-tablets-revealed-bahaullah/">_Additional Tablets and Extracts from Tablets Revealed by Bahá’u’lláh_</a>
    #[strum(serialize = "Additional Tablets and Extracts from Tablets Revealed by Bahá'u'lláh")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Additional Tablets and Extracts from Tablets Revealed by Bahá'u'lláh")
    // )]
    AdditionalTabletsAndExtractsBahaullah,
    /// from <a target="_blank" href="https://www.bahai.org/library/authoritative-texts/abdul-baha/additional-tablets-extracts-talks/">_Additional Tablets, Extracts, and Talks_ by 'Abdu'l‑Bahá</a>
    #[strum(serialize = "Additional Tablets, Extracts, and Talks by 'Abdu'l‑Bahá")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Additional Tablets, Extracts, and Talks by 'Abdu'l‑Bahá")
    // )]
    AdditionalTabletsExtractsAndTalksAbdulBaha,
}

impl TabletSource {
    pub fn title(&self) -> String {
        self.to_string()
    }

    pub fn author(&self) -> Author {
        match self {
            TabletSource::AdditionalTabletsAndExtractsBahaullah => Author::Bahaullah,
            TabletSource::AdditionalTabletsExtractsAndTalksAbdulBaha => Author::AbdulBaha,
        }
    }
}

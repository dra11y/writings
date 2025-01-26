use serde::{Deserialize, Serialize};

use crate::{WritingsTrait, author::Author};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
pub struct AdditionalTabletParagraph {
    pub source: TabletSource,
    pub ref_id: String,
    pub number: Option<u32>,
    pub paragraph_num: u32,
    pub text: String,
}

impl WritingsTrait for AdditionalTabletParagraph {
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

    fn paragraph_num(&self) -> u32 {
        self.paragraph_num
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
pub enum TabletSource {
    #[strum(serialize = "Additional Tablets and Extracts from Tablets Revealed by Bahá'u'lláh")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Additional Tablets and Extracts from Tablets Revealed by Bahá'u'lláh")
    // )]
    AdditionalTabletsAndExtractsBahaullah,
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

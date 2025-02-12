use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Author {
    /// The Báb
    #[strum(serialize = "The Báb")]
    // #[cfg_attr(feature = "poem", oai(rename = "The Báb"))]
    TheBab,
    /// Bahá’u’lláh
    #[strum(serialize = "Bahá’u’lláh")]
    // #[cfg_attr(feature = "poem", oai(rename = "Bahá’u’lláh"))]
    Bahaullah,
    /// ‘Abdu’l‑Bahá
    #[strum(serialize = "‘Abdu’l‑Bahá")]
    // #[cfg_attr(feature = "poem", oai(rename = "‘Abdu’l‑Bahá"))]
    AbdulBaha,
}

impl Author {
    pub fn name(&self) -> String {
        self.to_string()
    }
}

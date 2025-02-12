use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PrayerSource {
    #[strum(serialize = "Bahá'í Prayers")]
    // #[cfg_attr(feature = "poem", oai(rename = "Bahá'í Prayers"))]
    BahaiPrayers,
    #[strum(serialize = "Additional Prayers Revealed by Bahá’u’lláh")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Additional Prayers Revealed by Bahá’u’lláh")
    // )]
    AdditionalPrayersBahaullah,
    #[strum(serialize = "Additional Prayers Revealed by ‘Abdu’l‑Bahá")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Additional Prayers Revealed by ‘Abdu’l‑Bahá")
    // )]
    AdditionalPrayersAbdulBaha,
    #[strum(serialize = "Twenty-six Prayers Revealed by ‘Abdu’l‑Bahá")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Twenty-six Prayers Revealed by ‘Abdu’l‑Bahá")
    // )]
    TwentySixPrayersAbdulBaha,
    #[strum(serialize = "Bahá’í Prayers and Tablets for Children")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Bahá’í Prayers and Tablets for Children")
    // )]
    PrayersAndTabletsForChildren,
}

impl PrayerSource {
    pub fn title(&self) -> String {
        self.to_string()
    }
}

use serde::{Deserialize, Serialize};

/// A compilation of Bahá’í Prayers, the most well-known perhaps being [`PrayerSource::BahaiPrayers`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(
    feature = "utoipa",
    derive(writings_macros::ToEnumSchema),
    schema(descriptions = to_string)
)]
pub enum PrayerSource {
    /// <a href="https://www.bahai.org/library/authoritative-texts/prayers/bahai-prayers/" target="_blank">_Bahá’í Prayers: A Selection of Prayers Revealed by Bahá’u’lláh, the Báb, and ‘Abdu’l‑Bahá_</a>
    #[strum(serialize = "Bahá’í Prayers")]
    // #[cfg_attr(feature = "poem", oai(rename = "Bahá’í Prayers"))]
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

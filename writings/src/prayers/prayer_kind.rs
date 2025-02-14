use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

/// The "kind" or "category" of the prayer from <a href="https://www.bahai.org/library/authoritative-texts/prayers/bahai-prayers/" target="_blank">_Bahá’í Prayers_</a>.
/// [PrayerKind::Prologue] has been added to include the “Blessed is the spot...” and “Intone, O My servant...” selections at the beginning of the book.
#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PrayerKind {
    /// “Blessed is the spot...” and “Intone, O My servant...”
    #[strum(serialize = "Prologue")]
    Prologue,
    #[strum(serialize = "Obligatory Prayers")]
    // #[cfg_attr(feature = "poem", oai(rename = "Obligatory Prayers"))]
    Obligatory,
    #[strum(serialize = "General Prayers")]
    // #[cfg_attr(feature = "poem", oai(rename = "General Prayers"))]
    General,
    #[strum(serialize = "Occasional Prayers")]
    // #[cfg_attr(feature = "poem", oai(rename = "Occasional Prayers"))]
    Occasional,
    #[strum(serialize = "Special Tablets")]
    // #[cfg_attr(feature = "poem", oai(rename = "Special Tablets"))]
    Tablet,
}

impl PrayerKind {
    pub fn title(&self) -> String {
        self.to_string()
    }
}

impl TryFrom<&String> for PrayerKind {
    type Error = Option<PrayerKind>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for PrayerKind {
    type Error = Option<PrayerKind>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::iter().find(|k| k.title() == value).ok_or(None)
    }
}

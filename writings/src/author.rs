use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

// TODO: Include `ShoghiEffendi`, The `UniversalHouseOfJustice`, and `Institution()` in _this_ enum?

/// The three Central Figures of the Bahá’í Faith.
#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(
    feature = "utoipa",
    derive(writings_macros::ToEnumSchema),
    schema(descriptions = to_string)
)]
pub enum Author {
    // The Báb
    #[strum(serialize = "The Báb")]
    TheBab,
    // Bahá’u’lláh
    #[strum(serialize = "Bahá’u’lláh")]
    Bahaullah,
    // ‘Abdu’l‑Bahá
    #[strum(serialize = "‘Abdu’l‑Bahá")]
    AbdulBaha,
}

// Shoghi Effendi (The Guardian)
// ShoghiEffendi,
// The UniversalHouseOfJustice
// UniversalHouseOfJustice,

impl Author {
    pub fn name(&self) -> String {
        self.to_string()
    }
}

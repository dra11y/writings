use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, EnumIter};

use crate::{
    Author, BookParagraph, GleaningsParagraph, HiddenWord, MeditationParagraph, PrayerParagraph,
    TabletParagraph, WritingsTrait,
};

/// Allows enumeration of all Writings types in the crate.
/// See also the discriminants of this enum for use in APIs, etc.: [`WritingsType`]
#[derive(Debug, WritingsTrait, Clone, EnumDiscriminants, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[strum_discriminants(
    name(WritingsType),
    derive(EnumIter, Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "poem",
    derive(poem_openapi::Union),
    oai(one_of = true, discriminator_name = "type"),
    strum_discriminants(derive(poem_openapi::Enum))
)]
#[cfg_attr(
    feature = "utoipa",
    derive(utoipa::ToSchema),
    serde(untagged),
    strum_discriminants(derive(utoipa::ToSchema))
)]
pub enum Writings {
    // #[cfg_attr(feature = "utoipa", schema(title = "BookWriting"))]
    Book(BookParagraph),
    Gleaning(GleaningsParagraph),
    HiddenWord(HiddenWord),
    Prayer(PrayerParagraph),
    Meditation(MeditationParagraph),
    Tablet(TabletParagraph),
}

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for Writings {
    fn strings(&self) -> Vec<String> {
        match self {
            Writings::Book(b) => b.strings(),
            Writings::Gleaning(g) => g.strings(),
            Writings::HiddenWord(hw) => hw.strings(),
            Writings::Prayer(p) => p.strings(),
            Writings::Meditation(p) => p.strings(),
            Writings::Tablet(t) => t.strings(),
        }
    }
}

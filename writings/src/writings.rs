use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, EnumIter};

use crate::{
    Author, BookParagraph, CDBParagraph, GleaningsParagraph, HiddenWord, MeditationParagraph,
    PrayerParagraph, TabletParagraph, WritingsTrait,
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
    CDB(CDBParagraph),
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
            Writings::Book(w) => w.strings(),
            Writings::CDB(w) => w.strings(),
            Writings::Gleaning(w) => w.strings(),
            Writings::HiddenWord(w) => w.strings(),
            Writings::Prayer(w) => w.strings(),
            Writings::Meditation(w) => w.strings(),
            Writings::Tablet(w) => w.strings(),
        }
    }
}

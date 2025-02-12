mod additional_tablet;
pub use additional_tablet::{AdditionalTabletParagraph, TabletSource};
mod author;
pub use author::{Author, AuthorIter};
mod book;
pub use book::{BookParagraph, BookTitle};
mod gleanings;
pub use gleanings::GleaningsParagraph;
mod hidden_words;
pub use hidden_words::{HiddenWord, HiddenWordKind};
mod prayers;
pub use prayers::{PrayerKind, PrayerParagraph, PrayerSource};
mod embed_all;
#[cfg(feature = "_embed-any")]
pub use embed_all::EmbedAllTrait;
use writings_macros::WritingsTrait;
mod roman;
mod scraper_ext;
mod writings_visitor;
#[cfg(feature = "_visitors")]
pub use {
    gleanings::GleaningsVisitor, hidden_words::HiddenWordsVisitor, prayers::PrayersVisitor,
    writings_visitor::WritingsVisitor,
};

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumDiscriminants, EnumIter};
use thiserror::Error;

pub trait WritingsTrait: Sized + Clone {
    fn ref_id(&self) -> String;
    fn title(&self) -> String;
    fn subtitle(&self) -> Option<String>;
    fn author(&self) -> Author;
    fn number(&self) -> Option<u32>;
    fn paragraph_num(&self) -> u32;
    fn text(&self) -> String;
}

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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Writings {
    Book(BookParagraph),
    Gleanings(GleaningsParagraph),
    HiddenWord(HiddenWord),
    Prayer(PrayerParagraph),
    Tablet(AdditionalTabletParagraph),
}

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for Writings {
    fn strings(&self) -> Vec<String> {
        match self {
            // Writings::Book(b) => b.strings(),
            Writings::Book(_b) => vec![],
            Writings::Gleanings(g) => g.strings(),
            Writings::HiddenWord(hw) => hw.strings(),
            Writings::Prayer(p) => p.strings(),
            // Writings::Tablet(t) => t.strings(),
            Writings::Tablet(_t) => vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Citation {
    /// The reference ID from the official Bahá'í Reference Library:
    /// <https://www.bahai.org/r/`ref_id`>
    pub ref_id: String,

    /// The citation number as it appears in the text.
    pub number: u32,

    /// Absolute position in the text from the beginning of its paragraph.
    pub position: u32,

    /// The text of the footnote/endnote.
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ParagraphStyle {
    /// Regular text of the Writing
    Text,

    /// Primary Salutations (all caps)
    AllCapsSalutation,

    /// Salutations (italicized)
    Salutation,

    /// Only for instructions to the reader such as in the Obligatory Prayers
    Instruction,
}

pub type WritingsResult<T> = Result<T, WritingsError>;

#[derive(Debug, Error)]
pub enum WritingsError {
    SerdeValue(#[from] serde::de::value::Error),
}

impl Display for WritingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

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

#[derive(Debug, Clone, EnumDiscriminants, PartialEq, Eq, Serialize, Deserialize)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(name(WritingsType))]
#[cfg_attr(
    feature = "utoipa",
    derive(poem_openapi::Union),
    oai(one_of = true, discriminator_name = "type"),
    strum_discriminants(derive(poem_openapi::Enum))
)]
#[cfg_attr(
    feature = "poem",
    derive(poem_openapi::Union),
    oai(one_of = true, discriminator_name = "type"),
    strum_discriminants(derive(poem_openapi::Enum))
)]
pub enum Writings {
    Book(BookParagraph),
    Gleanings(GleaningsParagraph),
    HiddenWord(HiddenWord),
    Prayer(PrayerParagraph),
    Tablet(AdditionalTabletParagraph),
}

impl WritingsTrait for Writings {
    fn ref_id(&self) -> String {
        match self {
            Writings::Book(item) => item.ref_id(),
            Writings::Gleanings(item) => item.ref_id(),
            Writings::HiddenWord(item) => item.ref_id(),
            Writings::Prayer(item) => item.ref_id(),
            Writings::Tablet(item) => item.ref_id(),
        }
    }

    fn title(&self) -> String {
        match self {
            Writings::Book(item) => item.title(),
            Writings::Gleanings(item) => item.title(),
            Writings::HiddenWord(item) => item.title(),
            Writings::Prayer(item) => item.title(),
            Writings::Tablet(item) => item.title(),
        }
    }

    fn subtitle(&self) -> Option<String> {
        match self {
            Writings::Book(item) => item.subtitle(),
            Writings::Gleanings(item) => item.subtitle(),
            Writings::HiddenWord(item) => item.subtitle(),
            Writings::Prayer(item) => item.subtitle(),
            Writings::Tablet(item) => item.subtitle(),
        }
    }

    fn author(&self) -> Author {
        match self {
            Writings::Book(item) => item.author(),
            Writings::Gleanings(item) => item.author(),
            Writings::HiddenWord(item) => item.author(),
            Writings::Prayer(item) => item.author(),
            Writings::Tablet(item) => item.author(),
        }
    }

    fn number(&self) -> Option<u32> {
        match self {
            Writings::Book(item) => item.number(),
            Writings::Gleanings(item) => item.number(),
            Writings::HiddenWord(item) => item.number(),
            Writings::Prayer(item) => item.number(),
            Writings::Tablet(item) => item.number(),
        }
    }

    fn paragraph_num(&self) -> u32 {
        match self {
            Writings::Book(item) => item.paragraph_num(),
            Writings::Gleanings(item) => item.paragraph_num(),
            Writings::HiddenWord(item) => item.paragraph_num(),
            Writings::Prayer(item) => item.paragraph_num(),
            Writings::Tablet(item) => item.paragraph_num(),
        }
    }

    fn text(&self) -> String {
        match self {
            Writings::Book(item) => item.text(),
            Writings::Gleanings(item) => item.text(),
            Writings::HiddenWord(item) => item.text(),
            Writings::Prayer(item) => item.text(),
            Writings::Tablet(item) => item.text(),
        }
    }
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
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
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
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
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

//! The Bahá’í Sacred Writings for use in Rust projects and APIs.
//!
//! # Source
//!
//! All of the Writings are downloaded directly from <https://www.bahai.org/library>.
//! The downloaded HTML is included in the `html` folder.
//!
//! # Structure
//!
//! Each type of Writings has its own struct, e.g. `HiddenWord`, `PrayerParagraph`,
//! and `GleaningsParagraph`, which are hopefully explanatory of the unit the struct
//! represents. In this way, an excerpt from the Writings can be precisely referenced.
//!
//! The `ref_id` in each struct represents the exact reference ID at which the struct
//! points to at <https://www.bahai.org/r/`ref_id`>. For example, Persian Hidden Word #3
//! can be accessed directly at <https://www.bahai.org/r/607855955>. Hence, its struct
//! will have `ref_id` == "`607855955`". This is a `String`, **not** a `u32` or other
//! integer type, because www.bahai.org currently requires leading zeroes, and to ensure
//! future compatibility, it is not assumed integers of a fixed length will always be used.
//!
//! The `text` field of each struct is the exact plain text as extracted from the downloaded HTML.
//!
//! Other fields of each struct have their own documentation, depending on the type.
//!
//! # Usage
//!
//! ## Embed All
//!
//! When `::all()` is invoked on one of the structs implementing the `EmbedAllTrait`,
//! the HTML for that Work is parsed into the relevant structs once (very fast)
//! and stored in a `LazyLock<T>`.
//!
//! ## Example: Hidden Words
//!
//! ```
//! use writings::{HiddenWord, HiddenWordKind, EmbedAllTrait as _};
//!
//! let hw = HiddenWord::all()
//!     .iter()
//!     .find(|hw| hw.kind == HiddenWordKind::Persian && hw.number == Some(37))
//!     .cloned()
//!     .unwrap();
//!
//! assert_eq!(
//!     hw,
//!     HiddenWord {
//!         ref_id: "998408191".to_string(),
//!         kind: HiddenWordKind::Persian,
//!         number: Some(37),
//!         prelude: Some(concat!("In the first line of the Tablet it is recorded and written,",
//!             " and within the sanctuary of the tabernacle of God is hidden:").to_string()),
//!         invocation: Some("O My Servant!".to_string()),
//!         text: concat!("Abandon not for that which perisheth an everlasting dominion,",
//!             " and cast not away celestial sovereignty for a worldly desire. This is the river",
//!             " of everlasting life that hath flowed from the wellspring of the pen of the merciful;",
//!             " well is it with them that drink!").to_string(),
//!     }
//! );
//! ```
//!
//! ## Example: Gleanings
//!
//! ```
//! use writings::{GleaningsParagraph, EmbedAllTrait as _};
//!
//! let gleanings = GleaningsParagraph::all()
//!     .iter()
//!     .filter(|hw| hw.text.contains("all things visible and invisible"))
//!     .cloned()
//!     .collect::<Vec<_>>();
//!
//! assert_eq!(gleanings.len(), 3);
//!
//! let results = gleanings.iter().map(|g| (g.number, g.roman.as_str(), g.paragraph)).collect::<Vec<_>>();
//! assert_eq!(results, vec![
//!     (11, "XI", 3),
//!     (49, "XLIX", 1),
//!     (90, "XC", 2),
//! ]);
//!
//! assert!(gleanings[0].text.starts_with(concat!("No sooner had her voice reached that most exalted Spot",
//!     " than We made reply: “Render thanks unto thy Lord, O Carmel. The fire of thy separation from Me was",
//!     " fast consuming thee, when the ocean of My presence surged before thy face, cheering thine eyes and",
//!     " those of all creation, and filling with delight all things visible and invisible.")));
//! ```
//!
//! ## Example: Prayers
//!
//! ```
//! use writings::{PrayerKind, PrayerParagraph, EmbedAllTrait as _};
//!
//! let prayer = PrayerParagraph::all()
//!     .iter()
//!     .find(|p| {
//!         p.kind == PrayerKind::General
//!             && p.section
//!                 .iter()
//!                 .any(|s| s.contains("Western States")
//!             && p.paragraph == 2)
//!     })
//!     .cloned()
//!     .unwrap();
//!
//! assert_eq!(
//!     &prayer.text,
//!     concat!(
//!         "O God! O God! This is a broken-winged bird and his flight is very slow—assist him so that he may",
//!         " fly toward the apex of prosperity and salvation, wing his way with the utmost joy and happiness",
//!         " throughout the illimitable space, raise his melody in Thy Supreme Name in all the regions,",
//!         " exhilarate the ears with this call, and brighten the eyes by beholding the signs of guidance."
//!     )
//! );
//! ```
//!
//! # Languages
//!
//! Currently, only English is available in this crate. If reliable authoritative
//! sources can be found for other languages, it is hoped they will be added.
//! عربي (Arabic) and فارسی (Farsi / Persian) will be added if there is demand and
//! someone versed in these languages can help ensure the accuracy of the implementation.
//!
//! TODO: More docs...

mod additional_tablet;
pub use additional_tablet::{TabletParagraph, TabletSource};
mod author;
pub use author::{Author, AuthorIter};
mod book;
pub use book::{BookParagraph, BookTitle};
mod meditations;
pub use meditations::MeditationParagraph;

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
pub mod roman;
mod scraper_ext;
mod writings_visitor;
#[cfg(feature = "_visitors")]
pub use {
    gleanings::GleaningsVisitor, hidden_words::HiddenWordsVisitor, meditations::MeditationsVisitor,
    prayers::PrayersVisitor, writings_visitor::WritingsVisitor,
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
    fn paragraph(&self) -> u32;
    fn text(&self) -> String;
}

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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Writings {
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

/// A "footnote" or "endnote" embedded in a Text.
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

    /// Relative offset (in characters) of the citation,
    /// starting from 0 at the beginning of the text it's associated with.
    pub offset: u32,

    /// The text of the footnote/endnote.
    pub text: String,
}

/// Whether the struct text represents an invocation ("In the name of God, the Most Glorious!"),
/// an instruction to the reader, or "normal" text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ParagraphStyle {
    /// Regular Text of the Writing
    Text,

    /// Invocations (often displayed in ALL CAPS)
    Invocation,

    /// Instructions to the reader, such as those found in the Obligatory Prayers
    Instruction,
}

/// Alias for [`Result`] with [`WritingsError`] as the Error type.
pub type WritingsResult<T> = Result<T, WritingsError>;

/// An error type specific for this crate.
#[derive(Debug, Error)]
pub enum WritingsError {
    SerdeValue(#[from] serde::de::value::Error),
}

impl Display for WritingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

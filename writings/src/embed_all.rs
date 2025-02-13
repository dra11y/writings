#![cfg(feature = "_embed-any")]

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use crate::{
    GleaningParagraph, HiddenWord, MeditationParagraph, PrayerParagraph, Writings, WritingsTrait,
    writings_visitor::{VisitorAction, WritingsVisitor},
};

pub trait EmbedAllTrait: WritingsTrait {
    /// Lazily load and parse the embedded HTML for [`T`] and store it statically in memory.
    fn all() -> Arc<Vec<Self>>;

    /// Like `get_all()`, mapped with each record's `ref_id` as the key.
    fn all_map() -> Arc<HashMap<String, Self>>;
}

trait Storage: WritingsTrait {
    /// Embedded HTML file.
    const HTML: &str;

    /// The visitor used to parse the HTML.
    type Visitor: WritingsVisitor<Writings = Self>;

    /// The static cache for all records.
    fn once_all() -> &'static OnceLock<Arc<Vec<Self>>>;

    /// The static cache for all records mapped by `ref_id`.
    fn once_all_map() -> &'static OnceLock<Arc<HashMap<String, Self>>>;
}

/// We use a marker trait for single blanket exception until negative trait bounds are stable.
/// TODO: Must manually add impl for each new type of Writings.
trait NotWritingsEnum {}
impl NotWritingsEnum for HiddenWord {}
impl NotWritingsEnum for PrayerParagraph {}
impl NotWritingsEnum for GleaningParagraph {}
impl NotWritingsEnum for MeditationParagraph {}

impl<T> EmbedAllTrait for T
where
    T: 'static + WritingsTrait + Storage + NotWritingsEnum,
{
    fn all() -> Arc<Vec<Self>> {
        Self::once_all()
            .get_or_init(|| {
                let mut visitor = T::Visitor::default();
                visitor.parse_and_traverse(T::HTML);
                Arc::new(visitor.get_visited().to_vec())
            })
            .clone()
    }

    fn all_map() -> Arc<HashMap<String, Self>> {
        Self::once_all_map()
            .get_or_init(|| {
                let mut visitor = T::Visitor::default();
                visitor.parse_and_traverse(T::HTML);
                Arc::new(
                    visitor
                        .get_visited()
                        .iter()
                        .map(|it| (it.ref_id(), it.clone()))
                        .collect::<HashMap<_, _>>(),
                )
            })
            .clone()
    }
}

#[cfg(feature = "embed-all")]
impl EmbedAllTrait for Writings {
    fn all() -> Arc<Vec<Self>> {
        Self::once_all()
            .get_or_init(|| {
                let mut all = vec![];
                all.extend(
                    HiddenWord::all()
                        .iter()
                        .map(|it| Writings::HiddenWord(it.clone())),
                );
                all.extend(
                    PrayerParagraph::all()
                        .iter()
                        .map(|it| Writings::Prayer(it.clone())),
                );
                all.extend(
                    GleaningParagraph::all()
                        .iter()
                        .map(|it| Writings::Gleaning(it.clone())),
                );
                Arc::new(all)
            })
            .clone()
    }

    fn all_map() -> Arc<HashMap<String, Self>> {
        Self::once_all_map()
            .get_or_init(|| {
                Arc::new(
                    Self::all()
                        .iter()
                        .map(|it| (it.ref_id(), it.clone()))
                        .collect::<HashMap<_, _>>(),
                )
            })
            .clone()
    }
}

#[cfg(feature = "embed-all")]
impl Storage for Writings {
    type Visitor = NoOpVisitorForWritingsEnum;

    const HTML: &str = "";

    fn once_all() -> &'static OnceLock<Arc<Vec<Self>>> {
        static ALL: OnceLock<Arc<Vec<Writings>>> = OnceLock::new();
        &ALL
    }

    fn once_all_map() -> &'static OnceLock<Arc<HashMap<String, Self>>> {
        static ALL_MAP: OnceLock<Arc<HashMap<String, Writings>>> = OnceLock::new();
        &ALL_MAP
    }
}

#[cfg(feature = "embed-hidden-words")]
impl Storage for HiddenWord {
    type Visitor = crate::HiddenWordsVisitor;

    const HTML: &str = include_str!("../html/hidden_words.html");

    fn once_all() -> &'static OnceLock<Arc<Vec<Self>>> {
        static ALL: OnceLock<Arc<Vec<HiddenWord>>> = OnceLock::new();
        &ALL
    }

    fn once_all_map() -> &'static OnceLock<Arc<HashMap<String, Self>>> {
        static ALL_MAP: OnceLock<Arc<HashMap<String, HiddenWord>>> = OnceLock::new();
        &ALL_MAP
    }
}

#[cfg(feature = "embed-prayers")]
impl Storage for PrayerParagraph {
    type Visitor = crate::PrayersVisitor;

    const HTML: &str = include_str!("../html/prayers.html");

    fn once_all() -> &'static OnceLock<Arc<Vec<Self>>> {
        static ALL: OnceLock<Arc<Vec<PrayerParagraph>>> = OnceLock::new();
        &ALL
    }

    fn once_all_map() -> &'static OnceLock<Arc<HashMap<String, Self>>> {
        static ALL_MAP: OnceLock<Arc<HashMap<String, PrayerParagraph>>> = OnceLock::new();
        &ALL_MAP
    }
}

#[cfg(feature = "embed-gleanings")]
impl Storage for GleaningParagraph {
    type Visitor = crate::GleaningsVisitor;

    const HTML: &str = include_str!("../html/gleanings.html");

    fn once_all() -> &'static OnceLock<Arc<Vec<Self>>> {
        static ALL: OnceLock<Arc<Vec<GleaningParagraph>>> = OnceLock::new();
        &ALL
    }

    fn once_all_map() -> &'static OnceLock<Arc<HashMap<String, Self>>> {
        static ALL_MAP: OnceLock<Arc<HashMap<String, GleaningParagraph>>> = OnceLock::new();
        &ALL_MAP
    }
}

#[cfg(feature = "embed-meditations")]
impl Storage for MeditationParagraph {
    type Visitor = crate::MeditationsVisitor;

    const HTML: &str = include_str!("../html/meditations.html");

    fn once_all() -> &'static OnceLock<Arc<Vec<Self>>> {
        static ALL: OnceLock<Arc<Vec<MeditationParagraph>>> = OnceLock::new();
        &ALL
    }

    fn once_all_map() -> &'static OnceLock<Arc<HashMap<String, Self>>> {
        static ALL_MAP: OnceLock<Arc<HashMap<String, MeditationParagraph>>> = OnceLock::new();
        &ALL_MAP
    }
}

/// A no-op visitor for the `Writings` enum, which just aggregates the other types.
#[derive(Default, Debug)]
struct NoOpVisitorForWritingsEnum;

impl WritingsVisitor for NoOpVisitorForWritingsEnum {
    type Writings = Writings;

    const URL: &str = "";
    const EXPECTED_COUNT: usize = 0;

    fn get_visited(&self) -> &[Self::Writings] {
        &[]
    }

    fn visit(&mut self, _element: &scraper::ElementRef, _level: usize) -> VisitorAction {
        VisitorAction::Stop
    }
}

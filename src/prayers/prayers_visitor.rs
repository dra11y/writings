#![cfg(feature = "_visitors")]

use std::sync::LazyLock;

use scraper::{ElementRef, Selector};
use strum::IntoEnumIterator;

use crate::{
    ParagraphStyle,
    author::Author,
    scraper_ext::{ClassList, ElementExt as _},
    writings_visitor::{VisitorAction, WritingsVisitor},
};

use super::{PrayerKind, PrayerParagraph, PrayerSource};

#[derive(Debug, Default)]
pub struct PrayersVisitor {
    prayer_number: u32,
    prayers: Vec<PrayerParagraph>,
    current_section: Vec<String>,
    current_author: Option<Author>,
    paragraph_number: u32,
}

static AUTHOR_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".hb.ac").unwrap());

static ENDNOTES_CLASS: LazyLock<ClassList> = LazyLock::new(|| "bf wf".parse().unwrap());
static AUTHOR_CLASS: LazyLock<ClassList> = LazyLock::new(|| "hb ac".parse().unwrap());
static KIND_CLASS: LazyLock<ClassList> = LazyLock::new(|| "g c".parse().unwrap());
static SECTION_CLASS: LazyLock<ClassList> = LazyLock::new(|| "ub c l".parse().unwrap());
static SUBSECTION_CLASS: LazyLock<ClassList> =
    LazyLock::new(|| "xc jb c kf z nb zd ub".parse().unwrap());
static TEACHING_CLASS: LazyLock<ClassList> = LazyLock::new(|| "c kf z nb zd ub".parse().unwrap());
static INSTRUCTION_CLASSES: LazyLock<[ClassList; 2]> =
    LazyLock::new(|| ["cb".parse().unwrap(), "z".parse().unwrap()]);

impl WritingsVisitor for PrayersVisitor {
    type Writings = PrayerParagraph;

    const URL: &'static str = "https://www.bahai.org/library/authoritative-texts/prayers/bahai-prayers/bahai-prayers.xhtml";
    const EXPECTED_COUNT: usize = 981;

    fn get_visited(&self) -> &[Self::Writings] {
        &self.prayers
    }

    fn visit(&mut self, element: &ElementRef, html_level: usize) -> VisitorAction {
        if element.value().name() == "nav" {
            log::debug!("skip nav");
            return VisitorAction::SkipChildren;
        }

        if element.class_list().intersects(&ENDNOTES_CLASS) {
            log::debug!("skip notes / end");
            return VisitorAction::Stop;
        }

        // First check for sections
        if let Some((level, section)) = self.identify_section(element, html_level) {
            log::debug!("section: {section}, level: {level}, html level: {html_level}");
            self.current_section.truncate(level);
            self.current_section.push(section);
            return VisitorAction::SkipChildren;
        }

        // Then, if we find the Author, we're at the end of a prayer.
        // Reset in preparation for the next prayer.
        if identify_author(element).is_some() {
            self.current_author = None;
            self.paragraph_number = 0;
            return VisitorAction::SkipChildren;
        }

        let text = element.trimmed_text(1, true);

        // We must visit children here, as we start at the body level
        // and we don't assume depth of content paragraphs.
        if element.value().name() != "p" || text.is_empty() {
            return VisitorAction::VisitChildren;
        }

        // If we haven't found Author yet, find at the end of the next prayer.
        if self.current_author.is_none() {
            if let Some(author) = self.find_next_author(element) {
                self.current_author = Some(author);
                self.prayer_number += 1;
            }
        }

        // Finally check for paragraph content
        if let Some(paragraph) = self.extract_paragraph(element) {
            self.prayers.push(paragraph);
            return VisitorAction::SkipChildren;
        }

        VisitorAction::VisitChildren
    }
}

impl PrayersVisitor {
    fn extract_paragraph(&mut self, element: &ElementRef) -> Option<PrayerParagraph> {
        let author = self.current_author.as_ref()?;

        // Depth = 4 to ensure we get spans, etc.
        // TODO: handle superscripts/subscripts/footnotes?

        let exclude = element
            .select(&Selector::parse("sup").unwrap())
            .collect::<Vec<_>>();
        let text = element.filter_trimmed_text(4, true, &exclude);

        if text.is_empty() {
            return None;
        }

        self.paragraph_number += 1;

        let ref_id = self.get_ref_id(element);

        Some(PrayerParagraph {
            ref_id,
            number: self.prayer_number,
            author: *author,
            kind: self
                .current_section
                .iter()
                .take(1)
                .next()
                .and_then(|k| PrayerKind::try_from(k).ok()),
            section: self.current_section.iter().skip(1).cloned().collect(),
            source: PrayerSource::BahaiPrayers,
            paragraph_num: self.paragraph_number,
            style: determine_style(element),
            text,
        })
    }

    fn find_next_author(&self, element: &ElementRef) -> Option<Author> {
        let mut current = element.parent();
        while let Some(node) = current {
            if let Some(el) = ElementRef::wrap(node) {
                if let Some(author_el) = el.select(&AUTHOR_SELECTOR).next() {
                    if let Some(author) = identify_author(&author_el) {
                        return Some(author);
                    }
                }
            }
            current = node.parent();
        }
        None
    }

    fn identify_section(&self, element: &ElementRef, _level: usize) -> Option<(usize, String)> {
        let class_list = element.class_list();

        if class_list.includes(&SUBSECTION_CLASS) {
            return Some((2, element.trimmed_text(1, true)));
        }

        if class_list.includes(&TEACHING_CLASS) {
            return Some((3, element.trimmed_text(1, true)));
        }

        if class_list.includes(&SECTION_CLASS) {
            return Some((1, element.trimmed_text(1, true)));
        }

        if class_list == *KIND_CLASS {
            return Some((0, element.trimmed_text(1, true)));
        }

        None
    }
}

fn identify_author(element: &ElementRef) -> Option<Author> {
    if !element.class_list().includes(&AUTHOR_CLASS) {
        return None;
    }
    let text = element.trimmed_text(1, true);
    Author::iter().find(|a| text.contains(&a.name()))
}

fn determine_style(element: &ElementRef) -> ParagraphStyle {
    let class_list = element.class_list();

    if INSTRUCTION_CLASSES.iter().any(|c| class_list.includes(c))
        // (The Intercalary Days, February ...
        || element.trimmed_text(1, true).starts_with('(')
    {
        ParagraphStyle::Instruction
    } else {
        ParagraphStyle::Text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writings_visitor::test_helpers::*;

    #[tokio::test]
    async fn test_prayers_visitor() {
        test_visitor::<PrayersVisitor>().await;
    }
}

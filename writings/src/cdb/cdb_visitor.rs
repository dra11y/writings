#![cfg(feature = "_visitors")]
use super::CDBParagraph;
use crate::{
    ParagraphStyle,
    scraper_ext::{ClassList, ElementExt},
    writings_visitor::{VisitorAction, WritingsVisitor},
};
use scraper::{ElementRef, Selector};
use std::sync::LazyLock;

#[derive(Debug, Default)]
pub struct CDBVisitor {
    current_work: Option<String>,
    current_subtitle: Option<String>,
    current_index: u32,
    paragraphs: Vec<CDBParagraph>,
    in_work: bool,
}

static WORK_TITLE_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".ic .g").unwrap());
static WORK_SUBTITLE_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".ic .hb, .ic .j").unwrap());
static INVOCATION_CLASS: LazyLock<ClassList> = LazyLock::new(|| "w kf".parse().unwrap());
static PARAGRAPH_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("p").unwrap());
static FOOTER_CLASS: LazyLock<ClassList> = LazyLock::new(|| "wf".parse().unwrap());

impl WritingsVisitor for CDBVisitor {
    type Writings = CDBParagraph;

    const URL: &str = "https://www.bahai.org/library/authoritative-texts/bahaullah/call-divine-beloved/call-divine-beloved.xhtml";
    const EXPECTED_COUNT: usize = 150; // Approximate paragraph count

    fn get_visited(&self) -> &[Self::Writings] {
        &self.paragraphs
    }

    fn visit(&mut self, element: &scraper::ElementRef, _level: usize) -> VisitorAction {
        // Skip footer and notes
        if element.class_list() == *FOOTER_CLASS {
            return VisitorAction::Stop;
        }

        // Detect work titles
        if let Some(title) = self.get_work_title(element) {
            self.current_work = Some(title);
            self.current_subtitle = self.get_work_subtitle(element);
            self.current_index = 0;
            self.in_work = true;
            return VisitorAction::SkipChildren;
        }

        // Process paragraphs within works
        if self.in_work && element.name() == "p" {
            let style = if element.class_list() == *INVOCATION_CLASS {
                ParagraphStyle::Invocation
            } else {
                ParagraphStyle::Text
            };

            let ref_id = self.get_ref_id(element);
            self.current_index += 1;
            let text = element.trimmed_text(0, true);

            self.paragraphs.push(CDBParagraph {
                ref_id,
                work_title: self.current_work.clone().unwrap_or_default(),
                subtitle: self.current_subtitle.clone(),
                text,
                style,
                index: self.current_index,
            });
        }

        VisitorAction::VisitChildren
    }
}

impl CDBVisitor {
    fn get_work_title(&self, element: &ElementRef) -> Option<String> {
        if let Some(title) = element.select(&WORK_TITLE_SELECTOR).next() {
            return Some(title.trimmed_text(0, true));
        }
        None
    }

    fn get_work_subtitle(&self, element: &ElementRef) -> Option<String> {
        if let Some(subtitle) = element.select(&WORK_SUBTITLE_SELECTOR).next() {
            return Some(subtitle.trimmed_text(0, true));
        }
        None
    }
}

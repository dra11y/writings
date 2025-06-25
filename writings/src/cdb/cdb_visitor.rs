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

static POETRY_CONTAINER_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("span.dd").unwrap());
static CITATION_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("sup.ye").unwrap());
static WORK_TITLE_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".ic .g").unwrap());
static WORK_SUBTITLE_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".ic .hb, .ic .j").unwrap());
static PARAGRAPH_NUMBER_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a.td").unwrap());
static INVOCATION_CLASS: LazyLock<ClassList> = LazyLock::new(|| "ub w kf".parse().unwrap());

impl WritingsVisitor for CDBVisitor {
    type Writings = CDBParagraph;

    const URL: &str = "https://www.bahai.org/library/authoritative-texts/bahaullah/call-divine-beloved/call-divine-beloved.xhtml";
    const EXPECTED_COUNT: usize = 150; // Approximate paragraph count

    fn get_visited(&self) -> &[Self::Writings] {
        &self.paragraphs
    }

    fn visit(&mut self, element: &scraper::ElementRef, _level: usize) -> VisitorAction {
        // Skip notes
        if element.name() == "h2" && element.trimmed_text(1, false).trim() == "Notes" {
            println!("NOTES REACHED, STOP.");
            return VisitorAction::Stop;
        }

        // Detect work titles
        if let Some(title) = self.get_work_title(element) {
            if title == "Preface" {
                return VisitorAction::VisitChildren;
            }

            println!("TITLE: {title}");

            self.current_work = Some(title);
            self.current_subtitle = self.get_work_subtitle(element);
            self.current_index = 0;
            self.in_work = true;
            return VisitorAction::VisitChildren;
        }

        let is_poetry_span =
            element.name() == "span" && element.class_list().contains(&"dd".parse().unwrap());

        // Process paragraphs within works
        if self.in_work && element.name() == "p" {
            // Extract poetry container if exists
            let poetry_container = element.select(&POETRY_CONTAINER_SELECTOR).next();

            let style = match element.class_list() == *INVOCATION_CLASS {
                true => ParagraphStyle::Invocation,
                false => ParagraphStyle::Text,
            };

            // Skip paragraph numbers and citation markers
            let skip: Vec<ElementRef<'_>> = element
                .select(&PARAGRAPH_NUMBER_SELECTOR)
                .chain(element.select(&CITATION_SELECTOR)) // Add citation selector
                .chain(poetry_container.clone().into_iter())
                .collect();

            let text = element.trimmed_text_skip(1, true, &skip);

            // Get paragraph number only for <p> elements
            let number = element
                .select(&PARAGRAPH_NUMBER_SELECTOR)
                .next()
                .and_then(|el| el.trimmed_text(0, true).parse().ok());

            let ref_id = self.get_ref_id(element);
            self.current_index += 1;

            // Push main paragraph
            println!("# {number:?} - {style:?} - {text}");

            self.paragraphs.push(CDBParagraph {
                ref_id: ref_id.clone(),
                work_title: self.current_work.clone().unwrap_or_default(),
                subtitle: self.current_subtitle.clone(),
                number,
                text,
                style,
                index: self.current_index,
            });

            // Process poetry lines if they exist
            if let Some(pc) = poetry_container {
                for (i, line) in pc.select(&Selector::parse("span.ce").unwrap()).enumerate() {
                    self.current_index += 1;
                    let text = line.trimmed_text(1, true);
                    let line_ref_id = format!("{ref_id}-p{}", i + 1);

                    println!("# {number:?} - {:?} - {text}", ParagraphStyle::Blockquote);
                    self.paragraphs.push(CDBParagraph {
                        ref_id: line_ref_id,
                        work_title: self.current_work.clone().unwrap_or_default(),
                        subtitle: self.current_subtitle.clone(),
                        number: None,
                        text,
                        style: ParagraphStyle::Blockquote,
                        index: self.current_index,
                    });
                }
            }
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

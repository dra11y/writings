#![cfg(feature = "_visitors")]

use scraper::{ElementRef, Selector};

use crate::{Citation, WritingsTrait, scraper_ext::ElementExt as _};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitorAction {
    VisitChildren,
    SkipChildren,
    Stop,
}

pub fn resolve_citations(
    ref_id: &str,
    citations: &mut Vec<Citation>,
    citation_texts: &mut Vec<CitationText>,
) {
    for citation in citations.iter_mut() {
        let citation_ref_id = citation.ref_id.as_str();
        let Some(index) = citation_texts
            .iter()
            .position(|ct| ct.ref_id == citation_ref_id || ct.number == citation.number)
        else {
            panic!(
                "missing citation text for ref_id: {ref_id} , citation ref_id: {citation_ref_id} ,\nCITATION TEXTS: {citation_texts:#?}",
            );
        };
        citation.text = citation_texts.remove(index).text;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CitationText {
    number: u32,
    ref_id: String,
    text: String,
}

impl CitationText {
    pub fn text(&self) -> String {
        self.text.to_string()
    }

    pub fn matches(&self, citation: &Citation) -> bool {
        self.ref_id == citation.ref_id || self.number == citation.number
    }
}

pub trait WritingsVisitor: std::fmt::Debug + Send + Sync + Default {
    type Writings: WritingsTrait<Self::Writings> + PartialEq + Eq;

    const URL: &str;
    const EXPECTED_COUNT: usize;

    fn get_visited(&self) -> &[Self::Writings];

    fn get_citation_texts(&self, body_element: &ElementRef) -> Vec<CitationText> {
        let mut ct = vec![];
        for citation_link in body_element.select(&Selector::parse(".jf").unwrap()) {
            let number: u32 = citation_link.trimmed_text(1, true).parse().unwrap();
            let citation_parent = ElementRef::wrap(
                citation_link
                    .parent()
                    .expect("citation endnote missing parent"),
            )
            .expect("wrap citation parent");
            let ref_id = citation_parent
                .select(&Selector::parse("p a").unwrap())
                .next()
                .expect("citation ref_id element")
                .attr("id")
                .unwrap_or_else(|| panic!("citation without ref_id: {citation_parent:#?}"));
            let text = citation_parent
                .select(&Selector::parse("p").unwrap())
                .next()
                .expect("citation text element")
                .trimmed_text(1, true);
            let citation_text = CitationText {
                number,
                ref_id: ref_id.to_string(),
                text: text.to_string(),
            };
            println!("CITATION {number} {ref_id}: {text}");
            ct.push(citation_text);
        }
        ct
    }

    fn get_ref_id(&self, element: &ElementRef) -> String {
        element
            .select(&Selector::parse("a.sf").unwrap())
            .next()
            .expect("no ref id element for paragraph")
            .attr("id")
            .expect("no ref id for paragraph")
            .to_string()
    }

    fn visit(&mut self, element: &ElementRef, level: usize) -> VisitorAction;

    fn parse_and_traverse(&mut self, html: &str)
    where
        Self: Sized,
    {
        let document = scraper::Html::parse_document(html);
        let body = document
            .select(&Selector::parse("body").unwrap())
            .next()
            .unwrap();
        self.traverse(&body);
    }

    fn traverse(&mut self, element: &ElementRef)
    where
        Self: Sized,
    {
        traverse(self, element, 0);
    }
}

fn traverse<T: WritingsVisitor>(
    visitor: &mut T,
    element: &ElementRef,
    level: usize,
) -> VisitorAction {
    let action = visitor.visit(element, level);
    if action == VisitorAction::VisitChildren {
        for child in element.child_elements() {
            let action = traverse(visitor, &child, level + 1);
            if action == VisitorAction::Stop {
                return action;
            }
        }
    }
    action
}

#[cfg(test)]
pub mod test_helpers {
    use crate::WritingsTrait;

    use super::WritingsVisitor;

    pub async fn test_visitor<T: WritingsVisitor>(expect_texts: &[&str]) {
        let html = reqwest::get(T::URL).await.unwrap().text().await.unwrap();
        let mut visitor = T::default();
        visitor.parse_and_traverse(&html);
        let writings = visitor.get_visited();
        assert!(!writings.is_empty());
        assert_eq!(writings.len(), T::EXPECTED_COUNT);
        let texts = writings.iter().map(|w| w.text()).collect::<Vec<_>>();
        for expected_text in expect_texts {
            let contains = texts.iter().any(|text| text.contains(expected_text));
            assert!(
                contains,
                r#"None of the writings contains expected text: "{expected_text}""#
            );
        }
    }
}

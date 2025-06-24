#![cfg(feature = "_visitors")]
use std::collections::HashMap;

use scraper::{ElementRef, Selector};

use crate::{WritingsTrait, scraper_ext::ElementExt as _};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitorAction {
    VisitChildren,
    SkipChildren,
    Stop,
}

pub trait WritingsVisitor: std::fmt::Debug + Send + Sync + Default {
    type Writings: WritingsTrait<Self::Writings> + PartialEq + Eq;

    const URL: &str;
    const EXPECTED_COUNT: usize;

    fn get_visited(&self) -> &[Self::Writings];

    fn get_citation_texts(&self, body_element: &ElementRef) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for citation_link in body_element.select(&Selector::parse(".jf").unwrap()) {
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
            map.insert(ref_id.to_string(), text);
        }
        map
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
    use super::WritingsVisitor;

    pub async fn test_visitor<T: WritingsVisitor>() {
        let html = reqwest::get(T::URL).await.unwrap().text().await.unwrap();
        let mut visitor = T::default();
        visitor.parse_and_traverse(&html);
        let writings = visitor.get_visited();
        assert!(!writings.is_empty());
        assert_eq!(writings.len(), T::EXPECTED_COUNT);
    }
}

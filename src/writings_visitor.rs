#![cfg(feature = "_visitors")]
use scraper::{ElementRef, Selector};

use crate::WritingsTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitorAction {
    VisitChildren,
    SkipChildren,
    Stop,
}

pub trait WritingsVisitor: std::fmt::Debug + Send + Sync + Default {
    type Writings: WritingsTrait;

    const URL: &str;
    const EXPECTED_COUNT: usize;

    fn get_visited(&self) -> &[Self::Writings];

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

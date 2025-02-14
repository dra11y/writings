#![cfg(feature = "_visitors")]

use std::sync::LazyLock;

use crate::{
    scraper_ext::{ClassList, ElementExt},
    writings_visitor::{VisitorAction, WritingsVisitor},
};

use super::GleaningsParagraph;

#[derive(Debug, Default)]
pub struct GleaningsVisitor {
    number: u32,
    paragraph: u32,
    seen_first: bool,
    gleanings: Vec<GleaningsParagraph>,
}

static FOOTER_CLASS: LazyLock<ClassList> = LazyLock::new(|| "wf".parse().unwrap());
static ROMAN_NUMBER_CLASS: LazyLock<ClassList> = LazyLock::new(|| "c q".parse().unwrap());

impl WritingsVisitor for GleaningsVisitor {
    type Writings = GleaningsParagraph;

    const URL: &str = "https://www.bahai.org/library/authoritative-texts/bahaullah/gleanings-writings-bahaullah/gleanings-writings-bahaullah.xhtml";
    const EXPECTED_COUNT: usize = 716;

    fn get_visited(&self) -> &[Self::Writings] {
        &self.gleanings
    }

    fn visit(&mut self, element: &scraper::ElementRef, _level: usize) -> VisitorAction {
        if element.class_list() == *ROMAN_NUMBER_CLASS {
            self.seen_first = true;
            self.number += 1;
            self.paragraph = 0;
            assert_eq!(
                element
                    .trimmed_text(0, true)
                    .chars()
                    .filter(char::is_ascii_uppercase)
                    .collect::<String>(),
                crate::roman::to(self.number).unwrap()
            );
            return VisitorAction::SkipChildren;
        }

        if !self.seen_first {
            return VisitorAction::VisitChildren;
        }

        // Skip footer
        if element.class_list() == *FOOTER_CLASS {
            return VisitorAction::Stop;
        }

        if element.name() != "p" {
            return VisitorAction::VisitChildren;
        }

        self.paragraph += 1;
        let text = element.trimmed_text(4, true);
        let ref_id = self.get_ref_id(element);
        let paragraph = GleaningsParagraph {
            number: self.number,
            roman: crate::roman::to(self.number).unwrap(),
            paragraph: self.paragraph,
            text,
            ref_id,
        };
        self.gleanings.push(paragraph);

        VisitorAction::VisitChildren
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writings_visitor::test_helpers::*;

    #[tokio::test]
    async fn test_gleanings_visitor() {
        test_visitor::<GleaningsVisitor>().await;
    }
}

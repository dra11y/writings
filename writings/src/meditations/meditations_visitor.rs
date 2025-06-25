#![cfg(feature = "_visitors")]

use std::sync::LazyLock;

use crate::{
    scraper_ext::{ClassList, ElementExt},
    writings_visitor::{VisitorAction, WritingsVisitor},
};

use super::MeditationParagraph;

#[derive(Debug, Default)]
pub struct MeditationsVisitor {
    number: u32,
    paragraph: u32,
    seen_first: bool,
    meditation_text: Vec<MeditationParagraph>,
}

static FOOTER_CLASS: LazyLock<ClassList> = LazyLock::new(|| "wf".parse().unwrap());
static ROMAN_NUMBER_CLASS: LazyLock<ClassList> = LazyLock::new(|| "c q".parse().unwrap());

impl WritingsVisitor for MeditationsVisitor {
    type Writings = MeditationParagraph;

    const URL: &str = "https://www.bahai.org/library/authoritative-texts/bahaullah/prayers-meditations/prayers-meditations.xhtml";
    const EXPECTED_COUNT: usize = 877;

    fn get_visited(&self) -> &[Self::Writings] {
        &self.meditation_text
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
        let paragraph = MeditationParagraph {
            number: self.number,
            roman: crate::roman::to(self.number).unwrap(),
            paragraph: self.paragraph,
            text,
            ref_id,
        };
        self.meditation_text.push(paragraph);

        VisitorAction::VisitChildren
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writings_visitor::test_helpers::*;

    // TODO: Add expected texts
    const EXPECTED_TEXTS: &[&str] = &[];

    #[tokio::test]
    async fn test_meditations_visitor() {
        test_visitor::<MeditationsVisitor>(EXPECTED_TEXTS).await;
    }
}

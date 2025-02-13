#![cfg(feature = "_visitors")]

use std::sync::LazyLock;

use scraper::Selector;

use crate::{
    scraper_ext::{ClassList, ElementExt},
    writings_visitor::{VisitorAction, WritingsVisitor},
};

use super::{HiddenWord, HiddenWordKind};

#[derive(Debug, Default)]
pub struct HiddenWordsVisitor {
    seen_prologue: bool,
    prologue_ref_id: Option<String>,
    current_kind: HiddenWordKind,
    current_prelude: Option<String>,
    current_number: u32,
    hidden_words: Vec<HiddenWord>,
}

static TOP_INVOCATION_CLASS: LazyLock<ClassList> = LazyLock::new(|| "w".parse().unwrap());
static PROLOGUE_EPILOGUE_CLASS: LazyLock<ClassList> = LazyLock::new(|| "zd hb".parse().unwrap());
static PRELUDE_CLASS: LazyLock<ClassList> = LazyLock::new(|| "dd zd hb".parse().unwrap());
static HIDDEN_WORD_CLASS: LazyLock<ClassList> = LazyLock::new(|| "dd zd".parse().unwrap());

impl WritingsVisitor for HiddenWordsVisitor {
    type Writings = HiddenWord;

    const URL: &str = "https://www.bahai.org/library/authoritative-texts/bahaullah/hidden-words/hidden-words.xhtml";
    const EXPECTED_COUNT: usize = 155;

    fn get_visited(&self) -> &[Self::Writings] {
        &self.hidden_words
    }

    fn visit(&mut self, element: &scraper::ElementRef, _level: usize) -> VisitorAction {
        // Preludes
        if self.current_kind == HiddenWordKind::Persian
            // && [0, 19, 36, 47].contains(&self.current_number)
            && element.class_list() == *PRELUDE_CLASS
        {
            log::debug!(
                "PRELUDE: {} {}",
                self.current_number,
                element.trimmed_text(1, true)
            );
            self.current_prelude = Some(element.trimmed_text(1, true));
            return VisitorAction::SkipChildren;
        }

        // Top Invocation & Prologue
        if !self.seen_prologue
            && self.current_kind == HiddenWordKind::Arabic
            && element.name() == "p"
        {
            // Top Invocation
            if element.class_list() == *TOP_INVOCATION_CLASS {
                self.current_prelude = Some(element.trimmed_text(0, true));
                self.prologue_ref_id = Some(self.get_ref_id(element));
                return VisitorAction::SkipChildren;
            }

            // Prologue
            if element.class_list() == *PROLOGUE_EPILOGUE_CLASS {
                let text = element.trimmed_text(1, true);
                let hidden_word = HiddenWord {
                    ref_id: self.prologue_ref_id.take().unwrap(),
                    kind: HiddenWordKind::Arabic,
                    prelude: None,
                    number: None,
                    invocation: self.current_prelude.take(),
                    text,
                };
                self.hidden_words.push(hidden_word);
                self.seen_prologue = true;
                return VisitorAction::SkipChildren;
            }
        }

        // Eiplogue
        if self.current_kind == HiddenWordKind::Persian
            && element.name() == "p"
            && element.class_list() == *PROLOGUE_EPILOGUE_CLASS
        {
            let ref_id = self.get_ref_id(element);
            let text = element.trimmed_text(1, true);
            let hidden_word = HiddenWord {
                ref_id,
                kind: HiddenWordKind::Persian,
                prelude: None,
                number: None,
                invocation: None,
                text,
            };
            self.hidden_words.push(hidden_word);
            return VisitorAction::Stop;
        }

        // Transition to Part Two: From the Persian
        if self.current_kind == HiddenWordKind::Arabic
            && element.name() == "h2"
            && element.trimmed_text(0, true) == "Part Two"
        {
            self.current_kind = HiddenWordKind::Persian;
            self.current_number = 0;
            log::debug!("------------- RESET ---------------");
            return VisitorAction::SkipChildren;
        }

        if element.name() == "p" && element.class_list() == *HIDDEN_WORD_CLASS {
            let invocation = element
                .select(&Selector::parse("span.kf").unwrap())
                .next()
                .expect("missing Hidden Word salutation")
                .trimmed_text(1, true);
            let text = element.trimmed_text(0, true);
            let ref_id = self.get_ref_id(element);
            self.current_number += 1;
            let hidden_word = HiddenWord {
                ref_id,
                kind: self.current_kind.clone(),
                prelude: self.current_prelude.take(),
                number: Some(self.current_number),
                invocation: Some(invocation),
                text,
            };
            self.hidden_words.push(hidden_word);
        }

        VisitorAction::VisitChildren
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writings_visitor::test_helpers::*;

    #[tokio::test]
    async fn test_hidden_words_visitor() {
        test_visitor::<HiddenWordsVisitor>().await;
    }
}

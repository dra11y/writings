#![cfg(feature = "_scraper")]

use std::{collections::HashSet, str::FromStr, sync::LazyLock};

use regex::Regex;
use scraper::{ElementRef, Selector};

use crate::Citation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassList(HashSet<String>);

impl From<HashSet<String>> for ClassList {
    fn from(set: HashSet<String>) -> Self {
        Self(set)
    }
}

impl FromStr for ClassList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split_whitespace().map(|s| s.to_string()).collect()))
    }
}

impl<'a> FromIterator<&'a str> for ClassList {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Self(iter.into_iter().map(|s| s.to_string()).collect())
    }
}

impl FromIterator<String> for ClassList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl ClassList {
    pub fn includes(&self, other: &ClassList) -> bool {
        other.0.iter().all(|c| self.0.contains(c))
    }

    pub fn intersects(&self, other: &ClassList) -> bool {
        other.0.intersection(&self.0).next().is_some()
    }

    // pub fn is_disjoint(&self, other: &ClassList) -> bool {
    //     self.0.is_disjoint(&other.0)
    // }
}

pub trait ElementExt: Sized {
    fn trimmed_text_with_citations(
        &self,
        depth: usize,
        strip_newlines: bool,
        citations: &mut Option<&mut Vec<Citation>>,
    ) -> String;
    fn name(&self) -> &str;
    fn class_list(&self) -> ClassList;
    fn trimmed_text(&self, depth: usize, strip_newlines: bool) -> String;
}

static NEWLINE_WHITESPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s*\n\s*").unwrap());

impl ElementExt for ElementRef<'_> {
    fn name(&self) -> &str {
        self.value().name()
    }

    fn class_list(&self) -> ClassList {
        self.attr("class")
            .unwrap_or_default()
            .split_whitespace()
            .filter_map(|c| {
                let class = c.trim();
                if class.is_empty() {
                    None
                } else {
                    Some(class.to_string())
                }
            })
            .collect()
    }

    fn trimmed_text_with_citations(
        &self,
        max_depth: usize,
        strip_newlines: bool,
        citations: &mut Option<&mut Vec<Citation>>,
    ) -> String {
        trimmed_text_with_citations_inner(self, max_depth, strip_newlines, citations, 0)
    }

    fn trimmed_text(&self, max_depth: usize, strip_newlines: bool) -> String {
        self.trimmed_text_with_citations(max_depth, strip_newlines, &mut None)
    }
}

fn trimmed_text_with_citations_inner(
    element: &ElementRef<'_>,
    max_depth: usize,
    strip_newlines: bool,
    citations: &mut Option<&mut Vec<Citation>>,
    start_position: u32,
) -> String {
    let mut position = start_position;
    let mut trimmed = String::new();
    for child in element.children() {
        if let Some(child_ref) = ElementRef::wrap(child) {
            if child_ref.name() == "sup" {
                if let Some(citations) = citations.as_mut() {
                    let ref_id = child_ref
                        .select(&Selector::parse("a").unwrap())
                        .next()
                        .expect("no citation link")
                        .attr("href")
                        .expect("no href for citation")
                        .replace('#', "");
                    let num_text = child_ref.trimmed_text(1, true);
                    let number: u32 = num_text.parse().unwrap_or_else(|err| {
                        panic!("Invalid citation number: {num_text}, error: {err}")
                    });
                    println!("FOUND CITATION: ref_id: {ref_id}, number: {number}");
                    citations.push(Citation {
                        ref_id: ref_id.to_string(),
                        number,
                        position,
                        text: String::new(),
                    });
                }
                continue;
            }

            if max_depth > 0 {
                let child_text = trimmed_text_with_citations_inner(
                    &child_ref,
                    max_depth - 1,
                    strip_newlines,
                    citations,
                    position,
                );
                trimmed.push_str(&child_text);
                position += child_text.len() as u32;
            }

            continue;
        }
        if let Some(text) = child.value().as_text() {
            let text = if position == 0 {
                text.trim_start()
            } else {
                text
            };
            trimmed.push_str(text);
            position += text.len() as u32;
        }
    }
    if strip_newlines {
        trimmed = NEWLINE_WHITESPACE_RE
            .replace_all(&trimmed, " ")
            .trim()
            .to_string();
    }

    trimmed.trim_end().to_string()
}

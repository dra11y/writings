#![cfg(feature = "_scraper")]

use std::{collections::HashSet, str::FromStr, sync::LazyLock};

use regex::Regex;
use scraper::{ElementRef, Selector};

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
    fn filter_trimmed_text(&self, depth: usize, strip_newlines: bool, exclude: &[Self]) -> String;
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

    fn filter_trimmed_text(&self, depth: usize, strip_newlines: bool, exclude: &[Self]) -> String {
        let mut trimmed = String::new();
        for child in self.children() {
            if let Some(element_ref) = ElementRef::wrap(child) {
                if exclude.contains(&element_ref) {
                    continue;
                }
            }
            if let Some(text) = child.value().as_text() {
                trimmed.push_str(text);
            }
            if depth > 0 {
                if let Some(element) = ElementRef::wrap(child) {
                    trimmed.push_str(&element.trimmed_text(depth - 1, strip_newlines));
                }
            }
        }
        if strip_newlines {
            trimmed = NEWLINE_WHITESPACE_RE
                .replace_all(&trimmed, " ")
                .trim()
                .to_string();
        }
        trimmed.trim().to_string()
    }

    fn trimmed_text(&self, depth: usize, strip_newlines: bool) -> String {
        self.filter_trimmed_text(depth, strip_newlines, &[])
    }
}

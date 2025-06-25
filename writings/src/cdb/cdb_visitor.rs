#![cfg(feature = "_visitors")]
use super::CDBParagraph;
use crate::{
    Citation, ParagraphStyle,
    scraper_ext::{ClassList, ElementExt},
    writings_visitor::{CitationText, VisitorAction, WritingsVisitor},
};
use ego_tree::{NodeRef, iter::Edge};
use scraper::{ElementRef, Node, Selector};
use std::sync::LazyLock;

#[derive(Debug, Default)]
pub struct CDBVisitor {
    current_work: Option<String>,
    current_subtitle: Option<String>,
    paragraphs: Vec<CDBParagraph>,
    in_work: bool,
    citation_texts: Vec<CitationText>,
}

static POETRY_CONTAINER_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("span.dd").unwrap());
static LINE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("span.ce").unwrap());
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
    const EXPECTED_COUNT: usize = 205;

    fn get_visited(&self) -> &[Self::Writings] {
        &self.paragraphs
    }

    fn visit(&mut self, element: &scraper::ElementRef, _level: usize) -> VisitorAction {
        let name = element.name();

        if name == "body" {
            self.citation_texts = self.get_citation_texts(element);
        }

        // Skip notes
        if name == "h2" && element.trimmed_text(1, false).trim() == "Notes" {
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
            self.in_work = true;
            return VisitorAction::VisitChildren;
        }

        // Process paragraphs within works
        if !(self.in_work && element.name() == "p") {
            return VisitorAction::VisitChildren;
        }

        // Get paragraph ref_id
        let ref_id = self.get_ref_id(element);

        // Get paragraph number
        let number = element
            .select(&PARAGRAPH_NUMBER_SELECTOR)
            .next()
            .and_then(|el| el.trimmed_text(0, true).parse().ok());

        let mut offset: u32 = 0;

        let mut citations: Vec<Citation> = vec![];
        let mut lines: Vec<String> = vec![];
        let mut current_line = String::new();

        let mut ignored: Option<NodeRef<'_, Node>> = None;

        let mut style = match element.class_list() == *INVOCATION_CLASS {
            true => ParagraphStyle::Invocation,
            false => ParagraphStyle::Text,
        };

        for edge in element.traverse() {
            match edge {
                Edge::Open(node) => {
                    if ignored.is_some() {
                        // if let Some(el) = ElementRef::wrap(node) {
                        //     let debug_text = el.trimmed_text(4, false);
                        //     println!("IGNORED: {debug_text:?}")
                        // }
                        continue;
                    }

                    if let Some(text) = node.value().as_text() {
                        if text.trim().is_empty() {
                            continue;
                        }
                        // println!("FOUND TEXT: {text:?}");
                        current_line.push_str(text);
                        offset += text.len() as u32;
                        continue;
                    }

                    let Some(el) = ElementRef::wrap(node) else {
                        continue;
                    };

                    if [&PARAGRAPH_NUMBER_SELECTOR]
                        .iter()
                        .any(|sel| sel.matches(&el))
                    {
                        ignored = Some(node);
                        continue;
                    }

                    if let Some(mut citation) = el.get_citation(offset) {
                        if let Some(citation_text) =
                            self.citation_texts.iter().find(|ct| ct.matches(&citation))
                        {
                            // println!("FOUND CITATION: {citation:#?}");
                            citation.text = citation_text.text();
                            citations.push(citation);
                        } else {
                            panic!(
                                "Citation not found for ref_id {ref_id} in texts: {citation:#?}"
                            );
                        }
                        ignored = Some(node);
                        continue;
                    }

                    if POETRY_CONTAINER_SELECTOR.matches(&el) {
                        let text = lines.join("\n").trim().to_string();
                        println!("FOUND POETRY: {el:#?}\n\nPOETRY TEXT:\n{text:?}");

                        if !text.is_empty() {
                            self.paragraphs.push(CDBParagraph {
                                ref_id: ref_id.clone(),
                                work_title: self.current_work.clone().unwrap_or_default(),
                                subtitle: self.current_subtitle.clone(),
                                number,
                                text,
                                style,
                                citations: citations.clone(),
                            });
                        }

                        style = ParagraphStyle::Blockquote;
                        lines.clear();
                        citations.clear();
                        continue;
                    }
                }
                Edge::Close(node) => {
                    if ignored == Some(node) {
                        ignored = None;
                    }

                    let Some(el) = ElementRef::wrap(node) else {
                        continue;
                    };

                    if LINE_SELECTOR.matches(&el) {
                        current_line = current_line.trim().to_string();
                        if !current_line.is_empty() {
                            lines.push(current_line.clone());
                        }
                        current_line.clear();
                    }

                    if POETRY_CONTAINER_SELECTOR.matches(&el) {
                        style = ParagraphStyle::Text;
                    }
                }
            }
        }

        if !current_line.trim().is_empty() {
            current_line = current_line.trim().to_string();
            // println!("FINAL LINE: {current_line:?}");
            lines.push(current_line);
        }

        let text = lines.join("\n").trim().to_string();

        if text.is_empty() {
            panic!("text is empty for ref_id: {ref_id}");
        }

        // Push main paragraph
        println!("\n# {number:?} - {style:?} - {text}\ncitations: {citations:#?}");

        self.paragraphs.push(CDBParagraph {
            ref_id: ref_id.clone(),
            work_title: self.current_work.clone().unwrap_or_default(),
            subtitle: self.current_subtitle.clone(),
            number,
            text,
            style,
            citations: citations.clone(),
        });

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writings_visitor::test_helpers::*;

    const EXPECTED_TEXTS: &[&str] = &[
        "’Tis from Our rapture that the clouds",
        "’Tis from Our anthem that the mysteries",
        "Behold the fire of Moses, see His hand that shineth white;\nBehold the heart of Sinai—from Our hand all raining down.",
        "In the Name of God, the Merciful, the Compassionate!",
        "embodied in the most excellent Temple. And all to this end: that every man may testify",
        "thee from this abode of dust unto thy true and heavenly habitation in the midmost heart of mystic knowledge, and",
        "that he may enter into the realm of the spirit, which is the city of “but God”. Labour is needed",
        "He fleeth from both unbelief and faith, and findeth in deadly poison his heart’s relief. Wherefore ‘Aṭṭár saith:",
        "For the infidel, error—for the faithful, faith;\nFor ‘Aṭṭár’s heart, an atom of thy pain.",
        "O My brother! Until thou enter the Egypt of love, thou shalt never gaze upon the Joseph-like beauty of the Friend",
        "A lover feareth nothing and can suffer no harm: Thou seest him chill in the fire and dry in the sea.",
        "A lover is he who is chill in hellfire;\nA knower is he who is dry in the sea.",
        "Ne’er will love allow a living soul to tread its way;\nNe’er will the falcon deign to seize a lifeless prey.",
        "Love shunneth this world and that world too;\nIn him are lunacies seventy-and-two.",
    ];

    #[tokio::test]
    async fn test_cdb_visitor() {
        test_visitor::<CDBVisitor>(EXPECTED_TEXTS).await;
    }
}

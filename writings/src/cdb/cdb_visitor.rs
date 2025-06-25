#![cfg(feature = "_visitors")]
use super::CDBParagraph;
use crate::{
    ParagraphStyle,
    scraper_ext::{ClassList, ElementExt},
    writings_visitor::{CitationText, VisitorAction, WritingsVisitor, resolve_citations},
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
    citation_texts: Vec<CitationText>,
}

static POETRY_CONTAINER_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("span.dd").unwrap());
static LINE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("span.ce").unwrap());
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
    const EXPECTED_COUNT: usize = 249;

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
            self.current_index = 0;
            self.in_work = true;
            return VisitorAction::VisitChildren;
        }

        // Process paragraphs within works
        if self.in_work && element.name() == "p" {
            let mut citations = Vec::new();

            // Extract all poetry containers, if they exist.
            let poetry_containers = element
                .select(&POETRY_CONTAINER_SELECTOR)
                .collect::<Vec<_>>();

            // Get the lines of text that are immediate children of the paragraph
            // (Rashḥ-i-‘Amá)
            let mut lines = element
                .select(&LINE_SELECTOR)
                .filter(|el| el.parent().and_then(ElementRef::wrap).as_ref() == Some(element))
                .collect::<Vec<_>>();

            let style = match element.class_list() == *INVOCATION_CLASS {
                true => ParagraphStyle::Invocation,
                false => ParagraphStyle::Text,
            };

            // Skip paragraph numbers and citation markers
            let skip: Vec<ElementRef<'_>> = element
                .select(&PARAGRAPH_NUMBER_SELECTOR)
                .chain(element.select(&CITATION_SELECTOR)) // Add citation selector
                .chain(poetry_containers.clone())
                .collect();

            let text = lines
                .iter()
                .map(|line| {
                    line.trimmed_text_skip_with_citations(1, true, &skip, &mut Some(&mut citations))
                })
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();

            let ref_id = self.get_ref_id(element);
            self.current_index += 1;

            if text.is_empty() {
                panic!("text is empty for ref_id: {ref_id}");
            }

            // Get paragraph number only for <p> elements
            let number = element
                .select(&PARAGRAPH_NUMBER_SELECTOR)
                .next()
                .and_then(|el| el.trimmed_text(0, true).parse().ok());

            println!("RESOLVE CITATIONS #1, text:\n{text}");
            resolve_citations(&ref_id, &mut citations, &mut self.citation_texts);

            // Push main paragraph
            println!("\n# {number:?} - {style:?} - {text}\ncitations: {citations:#?}");

            self.paragraphs.push(CDBParagraph {
                ref_id: ref_id.clone(),
                work_title: self.current_work.clone().unwrap_or_default(),
                subtitle: self.current_subtitle.clone(),
                number,
                text,
                style,
                index: self.current_index,
                citations: citations.clone(),
            });

            // Process each poetry container as a separate stanza paragraph
            for (i, pc) in poetry_containers.into_iter().enumerate() {
                self.current_index += 1;

                // Collect all lines within this container
                let lines: Vec<String> = pc
                    .select(&Selector::parse("span.ce").unwrap())
                    .map(|line| line.trimmed_text(1, true))
                    .collect();

                // Join with newline separator
                let text = lines.join("\n");

                let stanza_ref_id = format!("{ref_id}-s{}", i + 1);

                println!("RESOLVE CITATIONS #2");
                resolve_citations(&ref_id, &mut citations, &mut self.citation_texts);

                println!("\n# None - Blockquote - {text}\ncitations: {citations:#?}");
                self.paragraphs.push(CDBParagraph {
                    ref_id: stanza_ref_id,
                    work_title: self.current_work.clone().unwrap_or_default(),
                    subtitle: self.current_subtitle.clone(),
                    number: None,
                    text,
                    style: ParagraphStyle::Blockquote,
                    index: self.current_index,
                    citations: citations.clone(),
                });
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
    async fn test_prayers_visitor() {
        test_visitor::<CDBVisitor>(EXPECTED_TEXTS).await;
    }
}

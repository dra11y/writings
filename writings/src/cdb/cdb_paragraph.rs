use crate::{WritingsTrait, WritingsType, author::Author, paragraph_style::ParagraphStyle};
use serde::{Deserialize, Serialize};

/// A paragraph from the Call of the Divine Beloved mystical works
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CDBParagraph {
    /// The reference ID from Bahá'í Reference Library
    pub ref_id: String,

    /// Title of the work (e.g., "The Seven Valleys")
    pub work_title: String,

    /// Subtitle of the work if present
    pub subtitle: Option<String>,

    /// The paragraph number within the work, if applicable
    pub number: Option<u32>,

    /// The actual text of this paragraph
    pub text: String,

    /// Paragraph style (normal text or invocation)
    pub style: ParagraphStyle,

    /// Index of paragraph within the work
    pub index: u32,
}

impl WritingsTrait<CDBParagraph> for CDBParagraph {
    fn ty(&self) -> WritingsType {
        WritingsType::CDB
    }

    fn ref_id(&self) -> String {
        self.ref_id.clone()
    }

    fn title(&self) -> String {
        self.work_title.clone()
    }

    fn subtitle(&self) -> Option<String> {
        self.subtitle.clone()
    }

    fn author(&self) -> crate::author::Author {
        Author::Bahaullah
    }

    fn number(&self) -> Option<u32> {
        None
    }

    fn paragraph(&self) -> u32 {
        self.index
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for CDBParagraph {
    fn strings(&self) -> Vec<String> {
        [self.ref_id.as_str(), &self.work_title, &self.text]
            .iter()
            .filter_map(|&s| {
                if s.is_empty() {
                    None
                } else {
                    Some(diacritics::remove_diacritics(s))
                }
            })
            .collect::<Vec<_>>()
    }
}

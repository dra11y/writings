use serde::{Deserialize, Serialize};

use crate::{WritingsTrait, author::Author};

/// A single paragraph from <a href="https://www.bahai.org/library/authoritative-texts/bahaullah/prayers-meditations/" target="_blank">_Prayers and Meditations by Bahá’u’lláh_</a>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(
    feature = "utoipa",
    derive(utoipa::ToSchema),
    schema(
        example = json!(MeditationParagraph {
            ref_id: "195857979".to_string(),
            number: 19,
            roman: "XIX".to_string(),
            paragraph: 1,
            text: "Praised be Thou, O Lord my God! I implore Thee, by Thy Most Great Name through Which Thou didst stir up Thy servants and build up Thy cities, and by Thy most excellent titles, and Thy most august attributes, to assist Thy people to turn in the direction of Thy manifold bounties, and set their faces towards the Tabernacle of Thy wisdom. Heal Thou the sicknesses that have assailed the souls on every side, and have deterred them from directing their gaze towards the Paradise that lieth in the shelter of Thy shadowing Name, which Thou didst ordain to be the King of all names unto all who are in heaven and all who are on earth. Potent art Thou to do as pleaseth Thee. In Thy hands is the empire of all names. There is none other God but Thee, the Mighty, the Wise.".to_string(),
        }),
    ),
)]
pub struct MeditationParagraph {
    /// The reference ID from the official Bahá'í Reference Library:
    /// <https://www.bahai.org/r/`ref_id`>
    pub ref_id: String,

    /// The _Prayers and Meditations_ number in decimal format.
    pub number: u32,

    /// The _Prayers and Meditations_ number in Roman Numeral format.
    pub roman: String,

    /// The number of the paragraph within the Prayer / Meditation, starting at 1.
    pub paragraph: u32,

    /// The actual Text of this paragraph of the Prayer / Meditation.
    pub text: String,
}

impl WritingsTrait for MeditationParagraph {
    fn ref_id(&self) -> String {
        self.ref_id.clone()
    }

    fn title(&self) -> String {
        "Prayers and Meditations".to_string()
    }

    fn subtitle(&self) -> Option<String> {
        None
    }

    fn author(&self) -> crate::author::Author {
        Author::Bahaullah
    }

    fn number(&self) -> Option<u32> {
        Some(self.number)
    }

    fn paragraph(&self) -> u32 {
        self.paragraph
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

impl MeditationParagraph {
    pub fn roman(&self) -> String {
        crate::roman::to(self.number).unwrap_or_else(|| {
            panic!("invalid Prayer / Meditation number -> Roman Numeral invalid: {self:#?} -- this error should never occur")
        })
    }
}

#[cfg(feature = "indicium")]
impl indicium::simple::Indexable for MeditationParagraph {
    fn strings(&self) -> Vec<String> {
        [
            self.ref_id.as_str(),
            self.roman.as_str(),
            &diacritics::remove_diacritics(&self.text),
        ]
        .iter()
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect()
    }
}

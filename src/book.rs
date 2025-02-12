use serde::{Deserialize, Serialize};

use crate::{WritingsTrait, author::Author};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "poem", derive(poem_openapi::Object))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BookParagraph {
    pub ref_id: String,
    pub title: BookTitle,
    pub subtitle: Option<String>,
    pub number: Option<u32>,
    pub paragraph_num: u32,
    pub text: String,
}

impl WritingsTrait for BookParagraph {
    fn ref_id(&self) -> String {
        self.ref_id.clone()
    }

    fn title(&self) -> String {
        self.title.to_string()
    }

    fn subtitle(&self) -> Option<String> {
        self.subtitle.clone()
    }

    fn author(&self) -> Author {
        self.title.author()
    }

    fn number(&self) -> Option<u32> {
        self.number
    }

    fn paragraph_num(&self) -> u32 {
        self.paragraph_num
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[cfg_attr(feature = "poem", derive(poem_openapi::Enum))]
pub enum BookTitle {
    // The Báb
    #[strum(serialize = "Selections from the Writings of the Báb")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Selections from the Writings of the Báb")
    // )]
    SelectionsFromTheWritingsOfTheBab,

    // Bahá’u’lláh
    #[strum(serialize = "Call of the Divine Beloved")]
    // #[cfg_attr(feature = "poem", oai(rename = "Call of the Divine Beloved"))]
    CallOfTheDivineBeloved,
    #[strum(serialize = "Days of Remembrance")]
    // #[cfg_attr(feature = "poem", oai(rename = "Days of Remembrance"))]
    DaysOfRemembrance,
    #[strum(serialize = "Epistle to the Son of the Wolf")]
    // #[cfg_attr(feature = "poem", oai(rename = "Epistle to the Son of the Wolf"))]
    EpistleToTheSonOfTheWolf,
    #[strum(serialize = "Gleanings from the Writings of Bahá’u’lláh")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Gleanings from the Writings of Bahá’u’lláh")
    // )]
    Gleanings,
    #[strum(serialize = "Kitáb-i-Aqdas")]
    // #[cfg_attr(feature = "poem", oai(rename = "Kitáb-i-Aqdas"))]
    KitabiAqdas,
    #[strum(serialize = "Kitáb-i-Íqán")]
    // #[cfg_attr(feature = "poem", oai(rename = "Kitáb-i-Íqán"))]
    KitabiIqan,
    #[strum(serialize = "Prayers and Meditations")]
    // #[cfg_attr(feature = "poem", oai(rename = "Prayers and Meditations"))]
    PrayersAndMeditations,
    #[strum(serialize = "Summons of the Lord of Hosts")]
    // #[cfg_attr(feature = "poem", oai(rename = "Summons of the Lord of Hosts"))]
    SummonsOfTheLordOfHosts,
    #[strum(serialize = "The Tabernacle of Unity")]
    // #[cfg_attr(feature = "poem", oai(rename = "The Tabernacle of Unity"))]
    TabernacleOfUnity,
    #[strum(serialize = "Tablets of Bahá’u’lláh")]
    // #[cfg_attr(feature = "poem", oai(rename = "Tablets of Bahá’u’lláh"))]
    TabletsOfBahaullah,

    // ‘Abdu’l‑Bahá
    #[strum(serialize = "Light of the World: Selected Tablets of ‘Abdu’l-Bahá")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Light of the World: Selected Tablets of ‘Abdu’l-Bahá")
    // )]
    LightOfTheWorld,
    #[strum(serialize = "Memorials of the Faithful")]
    // #[cfg_attr(feature = "poem", oai(rename = "Memorials of the Faithful"))]
    MemorialsOfTheFaithful,
    #[strum(serialize = "Paris Talks")]
    // #[cfg_attr(feature = "poem", oai(rename = "Paris Talks"))]
    ParisTalks,
    #[strum(serialize = "The Promulgation of Universal Peace")]
    // #[cfg_attr(feature = "poem", oai(rename = "The Promulgation of Universal Peace"))]
    PromulgationOfUniversalPeace,
    #[strum(serialize = "The Secret of Divine Civilization")]
    // #[cfg_attr(feature = "poem", oai(rename = "The Secret of Divine Civilization"))]
    SecretOfDivineCivilization,
    #[strum(serialize = "Selections from the Writings of ‘Abdu’l-Bahá")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Selections from the Writings of ‘Abdu’l-Bahá")
    // )]
    SelectionsFromTheWritingsOfAbdulBaha,
    #[strum(serialize = "Some Answered Questions")]
    // #[cfg_attr(feature = "poem", oai(rename = "Some Answered Questions"))]
    SomeAnsweredQuestions,
    #[strum(serialize = "Tablet to Dr. Auguste Forel")]
    // #[cfg_attr(feature = "poem", oai(rename = "Tablet to Dr. Auguste Forel"))]
    TabletToDrAugusteForel,
    #[strum(serialize = "Tablets of the Divine Plan")]
    // #[cfg_attr(feature = "poem", oai(rename = "Tablets of the Divine Plan"))]
    TabletsOfTheDivinePlan,
    #[strum(serialize = "Tablets to The Hague")]
    // #[cfg_attr(feature = "poem", oai(rename = "Tablets to The Hague"))]
    TabletsToTheHague,
    #[strum(serialize = "A Traveler’s Narrative")]
    // #[cfg_attr(feature = "poem", oai(rename = "A Traveler’s Narrative"))]
    ATravelersNarrative,
    #[strum(serialize = "Twelve Table Talks given by ‘Abdu’l‑Bahá in ‘Akká")]
    // #[cfg_attr(
    //     feature = "poem",
    //     oai(rename = "Twelve Table Talks given by ‘Abdu’l‑Bahá in ‘Akká")
    // )]
    TwelveTableTalks,
    #[strum(serialize = "Will and Testament of ‘Abdu’l‑Bahá")]
    // #[cfg_attr(feature = "poem", oai(rename = "Will and Testament of ‘Abdu’l‑Bahá"))]
    WillAndTestamentOfAbdulBaha,
}

impl BookTitle {
    pub fn title(&self) -> String {
        self.to_string()
    }

    pub fn author(&self) -> Author {
        match self {
            // The Báb
            BookTitle::SelectionsFromTheWritingsOfTheBab => Author::TheBab,

            // Bahá’u’lláh
            BookTitle::CallOfTheDivineBeloved => Author::Bahaullah,
            BookTitle::DaysOfRemembrance => Author::Bahaullah,
            BookTitle::EpistleToTheSonOfTheWolf => Author::Bahaullah,
            BookTitle::Gleanings => Author::Bahaullah,
            BookTitle::KitabiAqdas => Author::Bahaullah,
            BookTitle::KitabiIqan => Author::Bahaullah,
            BookTitle::PrayersAndMeditations => Author::Bahaullah,
            BookTitle::SummonsOfTheLordOfHosts => Author::Bahaullah,
            BookTitle::TabernacleOfUnity => Author::Bahaullah,
            BookTitle::TabletsOfBahaullah => Author::Bahaullah,

            // ‘Abdu’l-Bahá
            BookTitle::LightOfTheWorld => Author::AbdulBaha,
            BookTitle::MemorialsOfTheFaithful => Author::AbdulBaha,
            BookTitle::ParisTalks => Author::AbdulBaha,
            BookTitle::PromulgationOfUniversalPeace => Author::AbdulBaha,
            BookTitle::SecretOfDivineCivilization => Author::AbdulBaha,
            BookTitle::SelectionsFromTheWritingsOfAbdulBaha => Author::AbdulBaha,
            BookTitle::SomeAnsweredQuestions => Author::AbdulBaha,
            BookTitle::TabletToDrAugusteForel => Author::AbdulBaha,
            BookTitle::TabletsOfTheDivinePlan => Author::AbdulBaha,
            BookTitle::TabletsToTheHague => Author::AbdulBaha,
            BookTitle::ATravelersNarrative => Author::AbdulBaha,
            BookTitle::TwelveTableTalks => Author::AbdulBaha,
            BookTitle::WillAndTestamentOfAbdulBaha => Author::AbdulBaha,
        }
    }
}

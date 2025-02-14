use std::str::FromStr;

use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, IntoParams, ToSchema, Eq, PartialEq, Hash)]
/// Integer or Roman Numeral, e.g. `19` or `XIX`
#[schema(
    examples(19, "XIX"),
    value_type = String,
    format = Regex,
    pattern = r#"^\d{1,3}$|^M{0,3}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$"#,
)]
#[into_params(
    names("number"),
    style = Simple,
    parameter_in = Path,
)]
pub struct RomanNumber(#[param(example = "XIX")] pub u32);

impl<'de> Deserialize<'de> for RomanNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl FromStr for RomanNumber {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_ascii_digit()) {
            return match s.parse::<u32>() {
                Ok(num) => Ok(RomanNumber(num)),
                Err(_) => Err("Number too large"),
            };
        }
        writings::roman::from(s)
            .map(RomanNumber)
            .ok_or("Invalid number or Roman Numeral")
    }
}

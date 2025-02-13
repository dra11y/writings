use std::str::FromStr;

use serde::Deserialize;

pub fn get_from_env<T: FromStr>(name: &str, default: T) -> T {
    let Ok(val) = std::env::var(name) else {
        return default;
    };
    val.parse().unwrap_or(default)
}

pub fn split_path<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect())
}

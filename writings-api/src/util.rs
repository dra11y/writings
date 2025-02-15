use std::str::FromStr;

pub fn get_from_env<T: FromStr>(name: &str, default: T) -> T {
    let Ok(val) = std::env::var(name) else {
        return default;
    };
    val.parse().unwrap_or(default)
}

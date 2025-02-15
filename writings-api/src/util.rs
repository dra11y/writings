use std::str::FromStr;

use utoipa::openapi::{OpenApi, Tag};

pub fn get_from_env<T: FromStr>(name: &str, default: T) -> T {
    let Ok(val) = std::env::var(name) else {
        return default;
    };
    val.parse().unwrap_or(default)
}

pub fn openapi_with_tag(mut openapi: OpenApi, tag: &str) -> OpenApi {
    let tags = openapi.tags.get_or_insert_default();
    tags.push(Tag::new(tag));
    openapi
}

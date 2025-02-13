use std::{fs, path::Path};

use regex::Regex;
use writings::{
    GleaningsVisitor, HiddenWordsVisitor, MeditationsVisitor, PrayersVisitor, WritingsVisitor,
};

async fn download<T: WritingsVisitor>(name: &str) {
    let url = T::URL;
    let dir = Path::new(env!("CARGO_MANIFEST_PATH"))
        .parent()
        .unwrap()
        .join("../html")
        .canonicalize()
        .unwrap_or_else(|err| panic!("Failed to canonicalize path: {err:?}"));

    fs::create_dir_all(&dir)
        .unwrap_or_else(|err| panic!("Failed to create directory: {dir:?} - {err:?}"));

    let path = dir.join(name).with_extension("html");

    let timestamp_re = Regex::new(r#"<!-- Retrieved from.*-->"#).unwrap();
    let old_html = timestamp_re
        .replace(&fs::read_to_string(&path).unwrap_or_default(), "")
        .to_string();

    println!("\nFetching HTML from {url} ...");
    let html = reqwest::get(url).await.unwrap().text().await.unwrap();

    if html == old_html {
        println!("OK: {name}.html has not changed from last update.");
        return;
    }

    let now = chrono::Utc::now().to_rfc3339();
    let html_string = format!("<!-- Retrieved from {url} on {now} -->{html}",);

    let mut visitor = T::default();
    visitor.parse_and_traverse(&html_string);
    let writings = visitor.get_visited();

    if writings.is_empty() {
        panic!("CODE UPDATE REQUIRED: Visitor returned no Writings! {visitor:?}");
    }

    if writings.len() != T::EXPECTED_COUNT {
        panic!(
            "CODE UPDATE REQUIRED for visitor: {visitor:?}.\n\nUnexpected number of paragraphs: expected {}, found {}",
            T::EXPECTED_COUNT,
            writings.len()
        );
    }

    println!("Writing to {} ...", path.to_string_lossy());

    fs::write(&path, html_string)
        .unwrap_or_else(|err| panic!("Failed to write HTML file: {path:?} - {err:?}"));
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    download::<PrayersVisitor>("prayers").await;
    download::<HiddenWordsVisitor>("hidden_words").await;
    download::<GleaningsVisitor>("gleanings").await;
    download::<MeditationsVisitor>("meditations").await;
}

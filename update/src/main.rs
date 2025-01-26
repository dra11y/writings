use std::path::Path;

use writings::{GleaningsVisitor, HiddenWordsVisitor, PrayersVisitor, WritingsVisitor};

async fn get_html(url: &str) -> String {
    println!("Fetching HTML from {url} ...");
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

async fn download<T: WritingsVisitor>(name: &str) {
    let url = T::URL;
    let dir = Path::new(env!("CARGO_MANIFEST_PATH"))
        .parent()
        .unwrap()
        .join("../html")
        .canonicalize()
        .unwrap_or_else(|err| panic!("Failed to canonicalize path: {err:?}"));
    let path = dir.join(name).with_extension("html");
    let html = get_html(url).await;
    let now = chrono::Utc::now().to_rfc3339();
    let html_string = format!("<!-- Retrieved from {url} on {now} -->{html}",);

    let mut visitor = T::default();
    visitor.parse_and_traverse(&html_string);
    let writings = visitor.get_visited();
    assert!(
        !writings.is_empty(),
        "Visitor returned no Writings: {visitor:?}"
    );

    assert_eq!(
        writings.len(),
        T::EXPECTED_COUNT,
        "Unexpected number of Writings for visitor: {visitor:?}"
    );

    std::fs::create_dir_all(&dir)
        .unwrap_or_else(|err| panic!("Failed to create directory: {dir:?} - {err:?}"));
    println!("Writing to {} ...", path.to_string_lossy());
    std::fs::write(&path, html_string)
        .unwrap_or_else(|err| panic!("Failed to write HTML file: {path:?} - {err:?}"));
}

#[tokio::main]
async fn main() {
    color_eyre::install().expect("install color_eyre");
    download::<PrayersVisitor>("prayers").await;
    download::<HiddenWordsVisitor>("hidden_words").await;
    download::<GleaningsVisitor>("gleanings").await;
}

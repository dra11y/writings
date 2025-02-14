use std::sync::LazyLock;

use axum::{
    Json,
    extract::{Query, State},
};
use diacritics::remove_diacritics;
use indicium::simple::{
    AutocompleteType, RapidfuzzMetric, SearchIndex, SearchIndexBuilder, SearchType,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi as DeriveOpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{Author, EmbedAllTrait as _, Writings, WritingsTrait as _, WritingsType};

use crate::ApiResult;

#[derive(DeriveOpenApi)]
#[openapi(components(schemas()))]
pub struct SearchApiDoc;

pub fn router() -> OpenApiRouter {
    let index = build_search_index();
    OpenApiRouter::with_openapi(SearchApiDoc::openapi())
        .routes(routes!(get_search))
        .with_state(index)
}

fn build_search_index() -> SearchIndex<String> {
    let mut index: SearchIndex<String> = SearchIndexBuilder::default()
        .case_sensitive(false)
        .autocomplete_type(AutocompleteType::Global)
        .exclude_keywords(Some(
            ["thee", "thou", "thine", "hast"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .fuzzy_length(3)
        .max_autocomplete_options(9)
        .max_search_results(95)
        .rapidfuzz_metric(Some(RapidfuzzMetric::DamerauLevenshtein))
        .fuzzy_minimum_score(0.3)
        .build();

    println!("Loading Writings...");
    let writings = Writings::all_map();
    println!("Done loading Writings! Indexing...");
    writings.iter().for_each(|(ref_id, w)| {
        index.insert(ref_id, w);
    });
    println!("Done indexing Writings!");
    index
}

#[utoipa::path(
    get,
    path = "/",
    params(SearchQuery),
    responses(
        (status = OK, body = Vec<Writings>, description = "Prayer Paragraphs"),
    )
)]
pub async fn get_search(
    Query(query): Query<SearchQuery>,
    State(index): State<SearchIndex<String>>,
) -> ApiResult<Json<Vec<WritingsResult>>> {
    Ok(Json(search(&index, &query)))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WritingsResult {
    pub score: usize,
    pub ref_id: String,
    pub ty: WritingsType,
    pub author: Author,
    pub title: String,
    pub excerpt: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct SearchQuery {
    pub q: String,
}

static WORD_BOUNDARY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b").unwrap());
static SENTENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^.!?]+[.!?]?\s*").unwrap());

fn split_into_words(input: &str) -> Vec<String> {
    WORD_BOUNDARY_REGEX
        .split(input)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| diacritics::remove_diacritics(s).to_lowercase())
        .collect::<Vec<_>>()
}

fn search(index: &SearchIndex<String>, query: &SearchQuery) -> Vec<WritingsResult> {
    let query = remove_diacritics(&query.q);
    let keywords = split_into_words(&query);

    if keywords.is_empty() {
        return vec![];
    }

    // get all but the last keyword
    let must_match = &keywords[..keywords.len() - 1];
    let last_keyword = &keywords[keywords.len() - 1];

    let writings = Writings::all_map();

    let mut writings_results = index
        .search_type(&SearchType::Live, &query)
        .iter()
        .filter_map(|&ref_id| {
            writings.get(ref_id).and_then(|w| {
                SENTENCE_REGEX
                    .find_iter(&w.text())
                    .map(|m| m.as_str().trim())
                    .filter(|s| {
                        !s.is_empty() && {
                            let s = s.to_lowercase();
                            must_match.iter().all(|k| s.contains(k))
                        }
                    })
                    .filter_map(|s| {
                        let s_lower = s.to_lowercase();
                        let words = split_into_words(&s_lower);
                        // Calculate order score for must-match keywords
                        let mut current_kw = 0;
                        for word in &words {
                            if current_kw >= must_match.len() {
                                break;
                            }
                            if word.contains(&must_match[current_kw]) {
                                current_kw += 1;
                            }
                        }
                        let order_score = current_kw;

                        // Check last keyword presence
                        let last_present = words.iter().any(|word| word.contains(last_keyword));

                        // Calculate fuzzy match for last keyword if not present
                        let fuzzy_score = if last_present {
                            1.0f64
                        } else {
                            words
                                .iter()
                                .map(|word| {
                                    rapidfuzz::distance::lcs_seq::similarity(
                                        last_keyword.chars(),
                                        word.chars(),
                                    ) as f64
                                })
                                .max_by(|a, b| a.partial_cmp(b).unwrap())
                                .unwrap_or(0.0)
                        };

                        // Weighted scoring components
                        const ORDER_WEIGHT: usize = 1000;
                        const PRESENCE_WEIGHT: usize = 500;
                        const FUZZY_WEIGHT: f64 = 100.0;

                        let total_score = (order_score * ORDER_WEIGHT)
                            + (last_present as usize * PRESENCE_WEIGHT)
                            + (fuzzy_score * FUZZY_WEIGHT) as usize;

                        if total_score == 0 {
                            return None;
                        }

                        Some((total_score, s.trim().to_string()))
                    })
                    .max_by_key(|s| s.0)
                    .map(|excerpt| WritingsResult {
                        score: excerpt.0,
                        ref_id: ref_id.clone(),
                        ty: WritingsType::from(w),
                        author: w.author(),
                        title: w.title(),
                        excerpt: excerpt.1.clone(),
                    })
            })
        })
        .collect::<Vec<_>>();

    writings_results.sort_by_key(|w| -(w.score as i64));

    writings_results
}

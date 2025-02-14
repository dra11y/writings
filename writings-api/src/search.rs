use crate::{ApiResult, pagination::Pagination};
use axum::{
    Json,
    extract::{Query, State},
};
use axum_valid::Validated;
use diacritics::remove_diacritics;
use indicium::simple::{
    AutocompleteType, RapidfuzzMetric, SearchIndex, SearchIndexBuilder, SearchType,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use utoipa::{IntoParams, OpenApi as DeriveOpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use validify::Validify;
use writings::{Author, EmbedAllTrait as _, Writings, WritingsTrait as _, WritingsType};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas()))]
pub struct SearchApiDoc;

pub fn router() -> OpenApiRouter {
    let index = build_search_index();
    OpenApiRouter::with_openapi(SearchApiDoc::openapi())
        .routes(routes!(get_search))
        .with_state(index)
}

const DEFAULT_LIMIT: usize = 9;

#[derive(Debug, Deserialize, Validify, IntoParams, ToSchema)]
pub struct SearchQuery {
    pub q: String,
    #[validate(range(min = 1.0, max = 95.0))]
    #[serde(default = "default_limit")]
    #[param(default = default_limit, maximum = 95, minimum = 1)]
    pub limit: usize,
    #[serde(default)]
    #[param(default = 0)]
    pub offset: usize,
}

fn default_limit() -> usize {
    DEFAULT_LIMIT
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchResults {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub writings: Vec<WritingsResult>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WritingsResult {
    #[serde(skip)]
    pub score: usize,
    pub ty: WritingsType,
    pub author: Author,
    pub excerpt: String,
    #[serde(flatten)]
    pub writings: Writings,
}

impl WritingsResult {
    pub fn new(writings: Writings, excerpt: (usize, String)) -> Self {
        Self {
            score: excerpt.0,
            excerpt: excerpt.1,
            ty: writings.ty(),
            author: writings.author(),
            writings,
        }
    }
}

#[utoipa::path(
    get,
    path = "/",
    params(SearchQuery),
    responses(
        (status = OK, body = SearchResults, description = "Search results"),
    )
)]
#[axum::debug_handler]
pub async fn get_search(
    Validated(Query(query)): Validated<Query<SearchQuery>>,
    State(index): State<SearchIndex<String>>,
) -> ApiResult<Json<SearchResults>> {
    let mut writings = search(&index, &query);
    let total = writings.len();
    let start = query.offset.min(total);
    let end = (query.offset + query.limit).min(total);
    let writings: Vec<_> = writings.drain(start..end).collect();
    Ok(Json(SearchResults {
        pagination: Pagination {
            limit: query.limit,
            offset: query.offset,
            total: writings.len(),
        },
        writings,
    }))
}

static WORD_BOUNDARY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b").unwrap());
static SENTENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^.!?]+[.!?]?\s*").unwrap());

fn search(index: &SearchIndex<String>, query: &SearchQuery) -> Vec<WritingsResult> {
    const ORDER_WEIGHT: f64 = 800.0;
    const PROXIMITY_WEIGHT: f64 = 600.0;
    const POSITION_WEIGHT: f64 = 400.0;
    const EXACT_LAST_WEIGHT: f64 = 1000.0;
    const FUZZY_WEIGHT: f64 = 500.0;

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
                            let s_lower = s.to_lowercase();
                            must_match.iter().all(|k| s_lower.contains(k))
                        }
                    })
                    .filter_map(|s| {
                        let s_lower = s.to_lowercase();
                        let words = split_into_words(&s_lower);

                        // 1. Keyword positions and presence checks
                        let mut positions = Vec::with_capacity(keywords.len());

                        // Track must-match keywords
                        for kw in must_match {
                            if let Some(pos) = words.iter().position(|word| word.contains(kw)) {
                                positions.push(pos);
                            } else {
                                return None; // Should have been filtered earlier
                            }
                        }

                        // 2. Last keyword handling with minimum fuzzy threshold
                        let (last_present, last_pos, fuzzy_score) = {
                            if let Some(pos) =
                                words.iter().position(|word| word.contains(last_keyword))
                            {
                                (true, pos, 1.0)
                            } else {
                                // Require minimum fuzzy score of 0.7 similarity
                                let (best_score, best_pos) = words
                                    .iter()
                                    .enumerate()
                                    .map(|(pos, word)| {
                                        let score = rapidfuzz::distance::lcs_seq::similarity(
                                            last_keyword.chars(),
                                            word.chars(),
                                        )
                                            as f64
                                            / last_keyword.len().max(word.len()) as f64;
                                        (score, pos)
                                    })
                                    .max_by(|a, b| {
                                        a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
                                    })
                                    .unwrap_or((0.0, 0));

                                if best_score < 0.7 {
                                    return None;
                                }
                                (false, best_pos, best_score)
                            }
                        };
                        positions.push(last_pos);

                        // 3. Scoring components
                        // a. Order score (all keywords in sequence)
                        let order_score = positions
                            .windows(2)
                            .filter(|pair| pair[1] >= pair[0])
                            .count();

                        // b. Proximity score (average distance between consecutive keywords)
                        let proximity_score: f64 = positions
                            .windows(2)
                            .map(|pair| 1.0 / (pair[1].abs_diff(pair[0]) as f64 + 1.0))
                            .sum::<f64>()
                            / (positions.len() - 1).max(1) as f64;

                        // c. Position bonus (earlier first keyword better)
                        let position_score = 1.0 / (positions[0] as f64 + 1.0);

                        // d. Exact match bonus for last keyword
                        let exact_last_bonus = last_present as usize;

                        let total_score = (order_score as f64 * ORDER_WEIGHT)
                            + (proximity_score * PROXIMITY_WEIGHT)
                            + (position_score * POSITION_WEIGHT)
                            + (exact_last_bonus as f64 * EXACT_LAST_WEIGHT)
                            + (fuzzy_score * FUZZY_WEIGHT);

                        let total_score = total_score.round() as usize;

                        Some((total_score, s.trim().to_string()))
                    })
                    .max_by_key(|s| s.0)
                    .map(|excerpt| WritingsResult::new(w.clone(), excerpt))
            })
        })
        .collect::<Vec<_>>();

    writings_results.sort_by_key(|w| -(w.score as i64));

    writings_results
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

fn split_into_words(input: &str) -> Vec<String> {
    WORD_BOUNDARY_REGEX
        .split(input)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| diacritics::remove_diacritics(s).to_lowercase())
        .collect::<Vec<_>>()
}

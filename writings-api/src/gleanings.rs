use axum::{Json, extract::Path};
use utoipa::OpenApi as DeriveOpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, GleaningsParagraph};

use crate::{ApiError, ApiResult, roman_number::RomanNumber};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas(GleaningsParagraph, RomanNumber)))]
pub struct GleaningsApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(GleaningsApiDoc::openapi())
        .routes(routes!(get_all_gleanings))
        .routes(routes!(get_gleanings_number))
        .routes(routes!(get_gleanings_number_paragraph))
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, body = Vec<GleaningsParagraph>, description = "Prayer Paragraphs"),
    )
)]
pub async fn get_all_gleanings() -> ApiResult<Json<Vec<GleaningsParagraph>>> {
    Ok(Json(GleaningsParagraph::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{number}",
    // When the following `params` attribute is included:
    //   - the spec has a duplicate `number` param;
    //   - both `number` params have the "XIX" example pre-filled;
    //   - both `number` params show as "integer($int32)", which is wrong;
    //   - the first `number` param only accepts an integer, ignoring the regex.
    // When I make the param **not** a tuple, **and** the following `params` attribute is included:
    //   - the type is incorrect: "integer($int32)";
    //   - the example "XIX" shows;
    //   - the regex does **not** work, and Swagger will **only** accept an integer.
    // When the following `params` attribute is excluded:
    //   - the spec has one param (correct);
    //   - the `number` type shows as "string($regex)" (correct);
    //   - and the regex validation works in Swagger UI;
    //   - but there is **no** example in Swagger UI.
    params(RomanNumber),
    responses(
        (status = OK, body = Vec<GleaningsParagraph>, description = "Gleanings Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn get_gleanings_number(
    // If this is not a tuple, the spec has no parameters!
    Path(number): Path<RomanNumber>,
) -> ApiResult<Json<Vec<GleaningsParagraph>>> {
    Ok(Json(
        GleaningsParagraph::all()
            .iter()
            .filter(|p| p.number == number.0)
            .cloned()
            .collect(),
    ))
}

#[utoipa::path(
    get,
    path = "/{number}/{paragraph}",
    // When the following `params` attribute is included:
    //   - same duplication as above, but in order: `number`, `paragraph`, `number`;
    //   - as above, both `number` params have the "XIX" example and incorrect "integer($int32)" types;
    //   - as above, the first `number` param only accepts an integer, ignoring the regex.
    // When the following `params` attribute is excluded:
    //   - the spec has two params (correct);
    //   - as above, the `number` type shows as "string($regex)" (correct);
    //   - as above, the regex validation works in Swagger UI;
    //   - as above, there is **no** example in Swagger UI.
    // params(RomanNumber, ("paragraph" = u32, Path)),
    responses(
        (status = OK, body = GleaningsParagraph, description = "Gleanings Paragraph"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn get_gleanings_number_paragraph(
    Path((number, paragraph)): Path<(RomanNumber, u32)>,
) -> ApiResult<Json<GleaningsParagraph>> {
    Ok(Json(
        GleaningsParagraph::all()
            .iter()
            .find(|p| p.number == number.0 && p.paragraph == paragraph)
            .cloned()
            .ok_or(ApiError::NotFound)?,
    ))
}

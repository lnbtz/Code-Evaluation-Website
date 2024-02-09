pub mod response;

use core::str;

use crate::model::linter::parse_code;
use crate::model::rules::LineResult;
use askama::Template;
use axum::response::IntoResponse;
use response::HtmlTemplate;
#[derive(Template)]
#[template(path = "evaluation.html")]
pub struct EvaluationTemplate;

#[derive(Template)]
#[template(path = "suggestions.html")]
struct SuggestionsTemplate {
    suggestions: Vec<LineResult>,
}

pub async fn home_handler() -> impl IntoResponse {
    let template = EvaluationTemplate {};
    HtmlTemplate(template)
}

pub async fn eval(form: axum::Form<Code>) -> impl IntoResponse {
    let code = form.code.clone();
    let file_type = form.file_type.clone();
    let linter_result = parse_code(code, file_type);
    let template = SuggestionsTemplate {
        suggestions: linter_result,
    };
    HtmlTemplate(template)
}

#[derive(Debug, serde::Deserialize)]
pub struct Code {
    code: String,
    file_type: String,
}

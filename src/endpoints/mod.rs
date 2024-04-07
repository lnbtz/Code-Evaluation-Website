use crate::model::linter::parse_code;
use crate::model::rules::{load_rules, LineResult, Rule};
use askama::{Html, Template};
use askama_axum::Response;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::Form;

// region: templates
#[derive(Template)]
#[template(path = "evaluation.html")]
pub struct EvaluationTemplate;

#[derive(Template)]
#[template(path = "suggestions.html")]
struct SuggestionsTemplate {
    suggestions: Vec<LineResult>,
    code: Vec<(String, String)>,
}

#[derive(Template)]
#[template(path = "rules.html")]
struct ShowRulesTemplate {
    checkboxes: Vec<RuleCheckbox>,
}

// endregion: templates
// region: endpoints
pub async fn home() -> impl IntoResponse {
    let template = EvaluationTemplate {};
    HtmlTemplate(template)
}

pub async fn evaluation(form: Form<EvaluationInputForm>) -> impl IntoResponse {
    let code = form.code.clone();
    let file_type = form.file_type.clone();
    let rules_to_apply = form.rules.clone();
    let linter_result = parse_code(&code, file_type, rules_to_apply);
    let mirror_code = code
        .lines()
        .enumerate()
        .map(|(line_nr, line)| {
            let line_nr = line_nr + 1;
            if linter_result
                .iter()
                .any(|result| result.line == line_nr as i32)
            {
                ("highlight".to_string(), line.to_string())
            } else {
                ("normal".to_string(), line.to_string())
            }
        })
        .collect();
    let template = SuggestionsTemplate {
        suggestions: linter_result,
        code: mirror_code,
    };
    HtmlTemplate(template)
}

pub async fn rules(Query(file_type): Query<RulesForFileType>) -> impl IntoResponse {
    let rules = load_rules(file_type.file_type, vec![]);
    let checkboxes = build_checkboxes_data(rules);
    let template = ShowRulesTemplate { checkboxes };
    HtmlTemplate(template)
}

pub async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../../styles/styles.css").to_owned())
        .unwrap()
}

pub async fn image() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(axum::body::Body::from(
            include_bytes!("../../media/image.png").to_vec(),
        ))
        .unwrap()
}
// endregion: endpoints
// region: helpers
fn build_checkboxes_data(rules: Vec<Box<dyn Rule>>) -> Vec<RuleCheckbox> {
    let checkboxes = rules
        .iter()
        .map(|rule| RuleCheckbox {
            value: rule.get_name().to_lowercase(),
            name: rule.get_name().to_string(),
        })
        .collect();
    checkboxes
}

#[derive(Debug, serde::Deserialize)]
pub struct EvaluationInputForm {
    code: String,
    file_type: String,
    #[serde(rename = "rule")]
    rules: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct RulesForFileType {
    file_type: String,
}

struct RuleCheckbox {
    value: String,
    name: String,
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => axum::response::Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
// endregion: helpers

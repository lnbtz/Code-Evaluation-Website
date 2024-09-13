use crate::model::linter::evalute_code;
use crate::model::rules::{load_css_rules, load_html_rules, load_js_rules, LineResult, Rule};
#[allow(unused_imports)]
use askama::{Html, Template};
use askama_axum::Response;
use axum::extract;
use axum::http::StatusCode;
use axum::response::IntoResponse;

// region: templates
/// EvaluationTemplate is a struct that holds the data for the evaluation template
#[derive(Template)]
#[template(path = "evaluation.html")]
pub struct EvaluationTemplate;

/// SuggestionsTemplate is a struct that holds the data for the suggestions template
#[derive(Template)]
#[template(path = "suggestions.html")]
struct SuggestionsTemplate {
    /// suggestions is a vector of LineResult that can be displayed in the frontend as suggestions with its fields
    /// see suggestions.html for more information and check the askama documentation for more information on how to use templates
    suggestions: Vec<LineResult>,
    code: Vec<(String, String)>,
}

/// ShowRulesTemplate is a struct that holds the data for the rules template
#[derive(Template)]
#[template(path = "rules.html")]
struct ShowRulesTemplate {
    checkboxes: Vec<RuleCheckbox>,
}

// endregion: templates
// region: endpoints
/// home is the home endpoint that returns the home page
pub async fn home() -> impl IntoResponse {
    let template = EvaluationTemplate {};
    HtmlTemplate(template)
}

/// evaluation is the evaluation endpoint that returns the evaluation page
pub async fn evaluation(
    extract::Json(payload): extract::Json<EvaluationInputForm>,
) -> impl IntoResponse {
    let linter_result = evalute_code(&payload.code, payload.file_type, payload.rules);
    let mirror_code = payload
        .code
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

/// load all css rules
pub async fn css_rules() -> impl IntoResponse {
    let rules = load_css_rules(vec![]);
    let checkboxes = build_checkboxes_data(rules);
    let template = ShowRulesTemplate { checkboxes };
    HtmlTemplate(template)
}

/// load all js rules
pub async fn js_rules() -> impl IntoResponse {
    let rules = load_js_rules(vec![]);
    let checkboxes = build_checkboxes_data(rules);
    let template = ShowRulesTemplate { checkboxes };
    HtmlTemplate(template)
}

/// load all html rules
pub async fn html_rules() -> impl IntoResponse {
    let rules = load_html_rules(vec![]);
    let checkboxes = build_checkboxes_data(rules);
    let template = ShowRulesTemplate { checkboxes };
    HtmlTemplate(template)
}

/// styles is the styles endpoint that returns the styles
pub async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../../styles/styles.css").to_owned())
        .unwrap()
}

/// image is the image endpoint that returns the image
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

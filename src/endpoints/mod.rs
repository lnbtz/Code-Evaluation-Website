pub mod response;

use askama::Template;
use axum::response::IntoResponse;
use response::HtmlTemplate;

#[derive(Template)]
#[template(path = "evaluation.html")]
pub struct EvaluationTemplate;

#[derive(Template, Debug)]
#[template(path = "suggestions.html")]
struct SuggestionsTemplate {
    suggestions: Vec<String>,
}

pub async fn home_handler() -> impl IntoResponse {
    let template = EvaluationTemplate {};
    HtmlTemplate(template)
}

pub async fn eval(form: axum::Form<Code>) -> impl IntoResponse {
    let code = form.code.clone();
    println!("Code:\n {}", code);
    let file_type = form.file_type.clone();
    println!("File Type:\n {}", file_type);
    let suggestions: Vec<String> = vec![code, file_type];
    let template = SuggestionsTemplate { suggestions };
    println!("template:\n {:?}", template);
    HtmlTemplate(template)
}

#[derive(Debug, serde::Deserialize)]
pub struct Code {
    code: String,
    file_type: String,
}

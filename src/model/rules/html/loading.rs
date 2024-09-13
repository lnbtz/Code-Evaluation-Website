use crate::model::rules::LineResult;
use ::scraper::{Html, Selector};

use crate::model::ctx::Ctx;
use crate::model::rules::Severity;

use super::Rule;

pub struct Loading;

impl Rule for Loading {
    fn get_name(&self) -> &str {
        "Lazy-Loading"
    }

    fn get_description(&self) -> &str {
        // TODO add link to lazy loading
        "consinder lazy-loading images and scripts to improve performance."
    }

    fn apply(&self, ctx: &Ctx<'_>) -> Option<Vec<LineResult>> {
        Some(parse_lazy_loading(ctx.input))
    }
}

pub fn parse_lazy_loading(html: &str) -> Vec<LineResult> {
    let fragment = Html::parse_document(html);
    let mut result = lazy_loading_selector(&fragment, html, "img");
    let result2 = lazy_loading_selector(&fragment, html, "iframe");
    // join the two results
    result.extend(result2);
    result
}

fn lazy_loading_selector(fragment: &Html, html: &str, selector: &str) -> Vec<LineResult> {
    let selector = Selector::parse(selector).unwrap();
    let result = fragment
        .select(&selector)
        .filter(|element| element.value().attr("loading").is_none())
        .map(|element| {
            let line_column = get_line_and_column(html, &element);
            LineResult {
                severity: Severity::Warning,
                line: line_column.0,
                column: line_column.1,
                classification: "Lazy-Loading".to_string(),
                description: element.html(),
            }
        })
        .collect();
    result
}

fn get_line_and_column(html: &str, element: &scraper::ElementRef) -> (i32, i32) {
    let mut line_ctr = 0;
    for line in html.lines() {
        line_ctr += 1;
        if line.contains(element.value().attr("src").unwrap()) {
            return (
                line_ctr,
                line.find(element.value().attr("src").unwrap()).unwrap() as i32,
            );
        }
    }
    (0, 0)
}

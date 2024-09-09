use crate::model::rules::LineResult;
use css_minify::optimizations::Level;
use css_minify::optimizations::Minifier;

use super::Rule;

pub struct Minify;

impl Rule for Minify {
    fn get_name(&self) -> &str {
        "CSS-Minify"
    }
    fn get_description(&self) -> &str {
        "consider minifying the input to save css file size and thus bandwidth. click link to minify your css https://www.minifier.org/ or use a bundler like webpack"
    }
    fn apply(&self, input: &str) -> Option<std::vec::Vec<LineResult>> {
        let minified = Minifier::default()
            .minify(input, Level::Three)
            .unwrap_or_default();
        println!("minified: {}", minified);
        if minified != input.to_string() {
            Some(vec![LineResult {
                severity: crate::model::rules::Severity::Info,
                line: 1,
                column: 0,
                classification: self.get_name().to_string(),
                description: self.get_description().to_string(),
            }])
        } else {
            None
        }
    }
}

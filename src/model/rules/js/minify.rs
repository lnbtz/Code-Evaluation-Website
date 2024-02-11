use crate::model::rules::LineResult;

use super::Rule;

pub struct Minify;

impl Rule for Minify {
    fn get_name(&self) -> &str {
        "JS-Minify"
    }
    fn get_description(&self) -> &str {
        // TODO add link to minify js
        "consider minifying the input to save data size and thus bandwidth. click link to minify your js"
    }
    fn apply(&self, input: &str) -> Option<std::vec::Vec<LineResult>> {
        if input.lines().count() > 1 {
            Some(vec![LineResult {
                severity: crate::model::rules::Severity::Info,
                line: 1,
                column: 1,
                classification: self.get_name().to_string(),
                description: self.get_description().to_string(),
            }])
        } else {
            None
        }
    }
}

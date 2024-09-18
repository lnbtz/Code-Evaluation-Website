use crate::model::ctx::Ctx;
use crate::model::rules::LineResult;
use minifier::js::minify;

use super::Rule;

/// Rule to check for minification of javascript
pub struct Minify;
impl Rule for Minify {
    fn get_name(&self) -> &str {
        "JS-Minify"
    }
    fn get_description(&self) -> &str {
        "consider minifying the input to save javascript file size and thus bandwidth. click link to minify your javascript https://www.minifier.org/ or use a bundler like webpack"
    }
    fn apply(&self, ctx: &Ctx<'_>) -> Option<std::vec::Vec<LineResult>> {
        let input = ctx.java_script_ctx.as_ref().unwrap().input;
        let minified = minify(input);
        if minified.to_string() != input.to_string() {
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

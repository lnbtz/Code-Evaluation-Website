use crate::model::ctx::*;
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
    fn apply(&self, ctx: &Ctx<'_>) -> Option<std::vec::Vec<LineResult>> {
        // match the Ctx to CssCtx
        if let Ctx::CssCtx(css_ctx) = ctx {
            // minify the input
            let minified = Minifier::default()
                .minify(css_ctx.input, Level::Three)
                .unwrap_or_default();
            // check if the minified version is different from the input
            if minified != css_ctx.input.to_string() {
                // return the line result
                return Some(vec![LineResult {
                    severity: crate::model::rules::Severity::Info,
                    line: 0,
                    column: 0,
                    classification: self.get_name().to_string(),
                    description: self.get_description().to_string(),
                }]);
            } else {
                return None;
            }
        }
        None
    }
}

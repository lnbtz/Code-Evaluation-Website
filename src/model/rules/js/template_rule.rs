use crate::model::rules::LineResult;

use super::Rule;

/// This is a template rule that can be used to create new rules
/// To activate this rule, uncomment the export in src/model/rules/js/mod.rs
/// and add the rule to the load_js_rules function in src/model/rules/mod.rs by uncommenting the line there
pub struct TemplateRule;
impl Rule for TemplateRule {
    fn get_name(&self) -> &str {
        "JS-Template-Rule"
    }
    fn get_description(&self) -> &str {
        "some template rule"
    }
    fn apply(&self, ctx: &Ctx<'_>) -> Option<std::vec::Vec<LineResult>> {
        // add your rule logic here
        None
    }
}

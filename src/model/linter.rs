// parses the lines and returns the linter result
// LinterResult is a struct that contains the result of the linter
// each line is parsed and the result is stored in the LinterResult
use crate::model::ctx::*;
use crate::model::rules::*;
use oxc::allocator::Allocator;
use oxc::ast::ast::Program;
use oxc::parser::Parser;
use oxc::span::SourceType;

/// parse_code parses the code and returns the linter result
pub fn evalute_code(code: &str, file_type: String, rules_to_apply: Vec<String>) -> Vec<LineResult> {
    match file_type.as_str() {
        "js" => handle_js(code, rules_to_apply),
        "html" => handle_html(code, rules_to_apply),
        "css" => handle_css(code, rules_to_apply),
        _ => vec![LineResult {
            severity: Severity::Warning,
            line: 0,
            column: 0,
            classification: "error".to_string(),
            description: "Unsupported file type".to_string(),
        }],
    }
}

fn handle_js(code: &str, rules_to_apply: Vec<String>) -> Vec<LineResult> {
    // build js ast here
    let allocator = Allocator::default();
    let source_type = SourceType::from_path("javscript.js").unwrap();
    let ret = Parser::new(&allocator, code, source_type).parse();
    let program = ret.program;
    let ctx = Ctx {
        java_script_ctx: &Some(JavaScriptCtx {
            input: &code,
            program: &program,
        }),
        css_ctx: &None,
        html_ctx: &None,
    };
    let js_rules: Vec<Box<dyn Rule>> = load_js_rules(rules_to_apply);
    apply_rules(js_rules, &ctx)
}

fn handle_html(code: &str, rules_to_apply: Vec<String>) -> Vec<LineResult> {
    let ctx = Ctx {
        java_script_ctx: &None,
        css_ctx: &None,
        html_ctx: &Some(HtmlCtx { input: &code }),
    };

    let html_rules: Vec<Box<dyn Rule>> = load_html_rules(rules_to_apply);
    apply_rules(html_rules, &ctx)
}

fn handle_css(code: &str, rules_to_apply: Vec<String>) -> Vec<LineResult> {
    let ctx = Ctx {
        java_script_ctx: &None,
        css_ctx: &Some(CssCtx { input: &code }),
        html_ctx: &None,
    };
    let css_rules: Vec<Box<dyn Rule>> = load_css_rules(rules_to_apply);
    apply_rules(css_rules, &ctx)
}

fn apply_rules(rules_to_apply: Vec<Box<dyn Rule>>, ctx: &Ctx<'_>) -> Vec<LineResult> {
    let mut line_results: Vec<LineResult> = Vec::new();

    // apply rules to the code and store the results
    for rule in rules_to_apply {
        if let Some(result) = rule.apply(ctx) {
            line_results.extend(result);
        }
    }
    if line_results.is_empty() {
        line_results.push(LineResult {
            severity: Severity::Info,
            line: 0,
            column: 0,
            classification: "info".to_string(),
            description: "No issues found".to_string(),
        });
    }
    line_results
}

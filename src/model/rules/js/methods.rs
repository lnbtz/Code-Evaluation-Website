use crate::model::rules::LineResult;

use oxc::allocator::Allocator;
use oxc::ast::{AstKind, Visit};
use oxc::parser::Parser;
use oxc::span::{GetSpan, SourceType};

use super::Rule;

pub struct Methods;

impl Rule for Methods {
    fn get_name(&self) -> &str {
        "JS-Method-Calls"
    }
    fn get_description(&self) -> &str {
        // TODO add link to minify js
        "consider not using marked method calls and use checkout suggestions for alternatives"
    }
    fn apply(&self, input: &str) -> Option<std::vec::Vec<LineResult>> {
        let mut result = vec![];
        let allocator = Allocator::default();
        let source_text = input;
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = MethodFinder {
            function_name_spans: vec![],
            methods: vec![String::from("height")],
        };
        ast_pass.visit_program(&program);
        for (function_name, start, _end) in ast_pass.function_name_spans {
            let (line, column) = line_column(input, start);
            let classification = "bad method".to_string();
            let description = format!(
                "Consider not using the {}() method and checkout suggestions for alternatives",
                function_name
            );
            let line_result = LineResult {
                severity: crate::model::rules::Severity::Warning,
                line,
                column,
                classification,
                description,
            };
            result.push(line_result);
        }
        Some(result)
    }
}

#[derive(Debug, Default)]
struct MethodFinder {
    function_name_spans: Vec<(String, u32, u32)>,
    methods: Vec<String>,
}

impl<'a> Visit<'a> for MethodFinder {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        if let AstKind::CallExpression(call_expression) = kind {
            let method_name = if call_expression.callee.is_identifier_reference() {
                call_expression
                    .callee
                    .get_identifier_reference()
                    .unwrap()
                    .name
                    .to_string()
            } else {
                call_expression
                    .callee
                    .get_member_expr()
                    .unwrap()
                    .static_property_name()
                    .unwrap()
                    .to_string()
            };
            if self.methods.contains(&method_name) {
                self.function_name_spans.push((
                    method_name,
                    call_expression.callee.span().start,
                    call_expression.callee.span().end,
                ))
            };
        }
    }
}

fn line_column(input: &str, start: u32) -> (i32, i32) {
    let mut line = 1;
    let mut column = 0;
    for (i, c) in input.chars().enumerate() {
        if i as u32 == start {
            break;
        }
        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

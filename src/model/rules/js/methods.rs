use crate::model::rules::LineResult;

use oxc::allocator::Allocator;
use oxc::ast::{AstKind, Visit};
use oxc::parser::Parser;
use oxc::span::{GetSpan, SourceType};

use super::Rule;

/// Rule to check for method calls in javascript can be modified to check for other methods
/// checks for static and instance method calls
/// TODO: add more methods to check for that are for example not recommended because of performance reasons or because they are deprecated
#[derive(Default)]
pub struct Methods {
    function_name_spans: Vec<(String, u32, u32)>,
    /// methods to look for
    methods: Vec<String>,
}
impl Rule for Methods {
    fn get_name(&self) -> &str {
        "JS-Method-Calls"
    }
    fn get_description(&self) -> &str {
        // TODO add proper description
        "consider not using marked method calls and use checkout suggestions for alternatives"
    }
    fn apply(&self, input: &str) -> Option<std::vec::Vec<LineResult>> {
        let mut result = vec![];
        let mut method_finder = Methods {
            function_name_spans: vec![],
            methods: vec![String::from("height")],
        };
        // boilerplate for js parsing
        let allocator = Allocator::default();
        let source_text = input;
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        method_finder.visit_program(&program);

        // collect the results
        for (function_name, start, _end) in method_finder.function_name_spans {
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

impl<'a> Visit<'a> for Methods {
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

mod tests {

    #[test]
    fn test_visit_program_arrow_two_params_strict_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray = array.filter((item, index) => array.indexOf(item) === index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut method_finder = Methods {
            function_name_spans: vec![],
            methods: vec![String::from("indexOf")],
        };
        method_finder.visit_program(&program);
        assert_eq!(method_finder.function_name_spans.len(), 1);
        assert_eq!(method_finder.function_name_spans[0].0, "indexOf");
    }
}

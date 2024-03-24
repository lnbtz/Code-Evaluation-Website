use crate::model::rules::LineResult;

use oxc::allocator::Allocator;
use oxc::ast::ast::Argument::Expression;
use oxc::ast::ast::Expression::ArrowFunctionExpression;
use oxc::ast::ast::Expression::BinaryExpression;
use oxc::ast::ast::Expression::CallExpression;
use oxc::ast::ast::Expression::FunctionExpression;
use oxc::ast::ast::Statement::ExpressionStatement;
use oxc::ast::{AstKind, Visit};
use oxc::parser::Parser;
use oxc::span::{GetSpan, SourceType};
use oxc::syntax::operator::BinaryOperator::Equality;

use super::Rule;

pub struct Duplicates;

impl Rule for Duplicates {
    fn get_name(&self) -> &str {
        "JS-Duplicates"
    }
    fn get_description(&self) -> &str {
        // TODO add link to minify js
        "consider replacing this pattern with the with [...new Set()] pattern to improve performance and save energy"
    }
    fn apply(&self, input: &str) -> Option<std::vec::Vec<LineResult>> {
        let mut result = vec![];
        let allocator = Allocator::default();
        let source_text = input;
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = DuplicatesPatternFinder {
            function_name_spans: vec![],
            methods: vec![String::from("filter")],
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
struct DuplicatesPatternFinder {
    function_name_spans: Vec<(String, u32, u32)>,
    methods: Vec<String>,
}

impl<'a> Visit<'a> for DuplicatesPatternFinder {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        // TODO match identifier references in the method calls.
        // get the identifier of the filter method call and the indexOf method call and also the arguments of the filter method call
        // then match them all so the pattern matches

        // match method call
        if let AstKind::CallExpression(call_expression) = kind {
            // match method call 'filter'
            if is_filter_method(call_expression) {
                // match 'filter' call arguments and check for expression
                if let Expression(function_expression) = &call_expression.arguments[0] {
                    match &function_expression.get_inner_expression() {
                        // match arrow function expression
                        ArrowFunctionExpression(arrow_function_expression) => {
                            // handle arrow function expression
                            handle_arrow_function_expression(arrow_function_expression);
                        }
                        // match function expression
                        FunctionExpression(function_expression) => {
                            // handle function expression
                            handle_function_expression(function_expression);
                        }
                        _ => {}
                    }
                };
                self.function_name_spans.push((
                    String::from("filter"),
                    call_expression.callee.span().start,
                    call_expression.callee.span().end,
                ));
            }
        }
    }
}

fn handle_arrow_function_expression(
    arrow_function_expression: &oxc::allocator::Box<'_, oxc::ast::ast::ArrowFunctionExpression<'_>>,
) {
    dbg!(arrow_function_expression);
}

fn handle_function_expression(
    function_expression: &oxc::allocator::Box<'_, oxc::ast::ast::Function<'_>>,
) {
    if function_expression.params.items.len() >= 2 && function_expression.body.is_some() {
        // TODO handle unwrap()
        let body = function_expression.body.as_ref().unwrap();
        if let ExpressionStatement(expression_statement) = &body.statements[0] {
            if expression_statement.expression.is_binaryish() {
                handle_binary_expression(expression_statement);
            }
        }
    };
}

fn handle_binary_expression(
    expression_statement: &oxc::allocator::Box<'_, oxc::ast::ast::ExpressionStatement<'_>>,
) {
    if let BinaryExpression(binary_expression) =
        expression_statement.expression.get_inner_expression()
    {
        if binary_expression.operator == Equality
            && ((binary_expression.right.is_identifier_reference()
                && binary_expression.left.is_call_expression())
                || (binary_expression.right.is_call_expression()
                    && binary_expression.left.is_identifier_reference()))
        {
            if let CallExpression(call_expression) = &binary_expression.right {
                handle_call_expression(call_expression);
            } else if let CallExpression(call_expression) = &binary_expression.left {
                handle_call_expression(call_expression);
            }
        }
    }
}

fn handle_call_expression(
    call_expression: &oxc::allocator::Box<'_, oxc::ast::ast::CallExpression<'_>>,
) {
    // TODO handle unwrap()
    if call_expression
        .callee
        .get_member_expr()
        .unwrap()
        .static_property_name()
        .unwrap()
        == "indexOf"
    {
        println!(
            "{}",
            call_expression
                .callee
                .get_member_expr()
                .unwrap()
                .static_property_name()
                .unwrap()
        );
    }
}

fn is_filter_method(call_expression: &oxc::ast::ast::CallExpression<'_>) -> bool {
    // TODO handle unwrap()
    call_expression
        .callee
        .get_member_expr()
        .unwrap()
        .static_property_name()
        .unwrap()
        == "filter"
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

// test for the visit_program function
#[cfg(test)]
mod tests {
    use super::*;
    use oxc::allocator::Allocator;

    use oxc::ast::Visit;
    use oxc::parser::Parser;
    use oxc::span::SourceType;

    #[test]
    fn test_visit_program() {
        let allocator = Allocator::default();
        let source_text = "uniqueArray = a.filter(function(item, pos) { a.indexOf(item) == pos })";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = DuplicatesPatternFinder {
            function_name_spans: vec![],
            methods: vec![String::from("filter")],
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.function_name_spans.len(), 1);
        assert_eq!(ast_pass.function_name_spans[0].0, "filter");
    }

    #[test]
    fn test_visit_program1() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray = array.filter((value, index, self) => self.indexOf(value) === index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = DuplicatesPatternFinder {
            function_name_spans: vec![],
            methods: vec![String::from("filter")],
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.function_name_spans.len(), 1);
        assert_eq!(ast_pass.function_name_spans[0].0, "filter");
    }
}

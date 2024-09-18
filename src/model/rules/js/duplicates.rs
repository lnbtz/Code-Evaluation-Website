use crate::model::rules::LineResult;

use oxc::ast::ast::BindingPatternKind::BindingIdentifier;
use oxc::ast::ast::Expression::ArrowFunctionExpression;
use oxc::ast::ast::Expression::BinaryExpression;
use oxc::ast::ast::Expression::CallExpression;
use oxc::ast::ast::Expression::FunctionExpression;
use oxc::ast::ast::Expression::Identifier;

use super::Rule;
use crate::model::ctx::Ctx;
use oxc::ast::ast::Statement::ExpressionStatement;
use oxc::ast::ast::Statement::ReturnStatement;
use oxc::ast::visit::walk::walk_call_expression;
use oxc::ast::Visit;

use oxc::span::GetSpan;
use oxc::syntax::operator::BinaryOperator::Equality;
use oxc::syntax::operator::BinaryOperator::StrictEquality;
/// This rule is used to find filter method calls that remove duplicates from an array
#[derive(Debug, Default)]
pub struct Duplicates {
    /// function name, start, end
    matches: Vec<(String, u32, u32)>,
    array_identifier: String,
    item: String,
    pos: String,
}

impl Rule for Duplicates {
    fn get_name(&self) -> &str {
        "JS-Duplicates"
    }
    fn get_description(&self) -> &str {
        "Wenn Sie hier statt der 'filter' Methode die '[...new Set()]' Methode verwenden würden, um Duplikate aus dem Array zu löschen, können sie Rechenzeit (ca. Faktor 100) und Energie sparen (ca. Faktor 1000)"
    }
    fn apply(&self, ctx: &Ctx<'_>) -> Option<std::vec::Vec<LineResult>> {
        if let Ctx::JavaScriptCtx(js_ctx) = ctx {
            let ast = js_ctx.program;
            let mut result = vec![];
            let mut duplicates: Duplicates = Duplicates::default();
            duplicates.visit_program(ast);

            // iterate over the matches and create LineResult objects
            for (_function_name, start, _end) in &duplicates.matches {
                let (line, column) = Duplicates::line_column(js_ctx.input, *start);
                let classification = self.get_name().to_string();
                let description = self.get_description().to_string();
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
        } else {
            None
        }
    }
}

impl<'a> Visit<'a> for Duplicates {
    // entrypoint for the visitor pattern
    fn visit_call_expression(&mut self, expr: &oxc::ast::ast::CallExpression<'a>) {
        // extract the method name, array identifier and arguments from the call expression
        let (method_name, array_identifier, arguments) = {
            (
                &expr
                    .callee
                    .get_inner_expression()
                    .get_member_expr()
                    .unwrap()
                    .static_property_name(),
                &expr
                    .callee
                    .get_inner_expression()
                    .get_member_expr()
                    .unwrap()
                    .object()
                    .get_identifier_reference()
                    .unwrap()
                    .name,
                &expr.arguments,
            )
        };
        // check if the method name is 'filter' and has the correct number of arguments
        if method_name.unwrap_or("not filter") == "filter" && arguments.len() == 1 {
            // assign the array identifier for later matching
            self.array_identifier = array_identifier.to_string();
            // match whether it is a function expression  call or an arrow function expression call
            match &arguments[0].to_expression() {
                // handle arrow function expression
                ArrowFunctionExpression(arrow_function_expression) => {
                    self.handle_arrow_function_expression(arrow_function_expression);
                }
                // handle function expression
                FunctionExpression(function_expression) => {
                    self.handle_function_expression(function_expression);
                }
                _ => {}
            };
        }
        // continue walking the AST
        walk_call_expression(self, expr);
    }
}

impl Duplicates {
    // region: handlers

    /// Handle the arrow function expression and check its bodies validity for further processing
    fn handle_arrow_function_expression<'a>(
        &mut self,
        arrow_function_expression: &'a oxc::allocator::Box<
            'a,
            oxc::ast::ast::ArrowFunctionExpression<'a>,
        >,
    ) {
        let function_body = &arrow_function_expression.body;
        if is_valid_expression_statement(arrow_function_expression, function_body) {
            self.extract_binding_identifiers_from_arrow_function(arrow_function_expression);
            self.handle_expression_statement(function_body);
        } else if is_valid_return_statement(arrow_function_expression, function_body) {
            self.extract_binding_identifiers_from_arrow_function(arrow_function_expression);
            self.handle_return_statement(function_body);
        };
    }

    /// Handle the function expression and check its bodies validity for further processing
    fn handle_function_expression<'a>(
        &mut self,
        function_expression: &'a oxc::allocator::Box<'a, oxc::ast::ast::Function<'a>>,
    ) {
        if function_expression.params.items.len() >= 2 && function_expression.body.is_some() {
            self.extract_binding_identifiers_from_function_expression(function_expression);
            if let Some(body) = &function_expression.body {
                if body.statements.len() > 0 {
                    self.handle_return_statement(body);
                }
            }
        };
    }

    /// Handle the expression statement and the internal binary expression
    fn handle_expression_statement<'a>(
        &mut self,
        function_body: &'a oxc::allocator::Box<'a, oxc::ast::ast::FunctionBody<'a>>,
    ) {
        if let ExpressionStatement(binary_expression) = &function_body.statements[0] {
            if let BinaryExpression(binary_expression) = &binary_expression.expression {
                self.handle_binary_expression(binary_expression);
            }
        }
    }

    /// Handle the return statement and the internal binary expression
    fn handle_return_statement<'a>(
        &mut self,
        function_body: &'a oxc::allocator::Box<'a, oxc::ast::ast::FunctionBody<'a>>,
    ) {
        if let ReturnStatement(return_statement) = &function_body.statements[0] {
            if let Some(BinaryExpression(binary_expression)) = &return_statement.argument {
                self.handle_binary_expression(binary_expression);
            }
        }
    }

    /// Handle the binary expression and check if it is a valid indexOf method call
    fn handle_binary_expression<'a>(
        &mut self,
        binary_expression: &'a oxc::allocator::Box<'a, oxc::ast::ast::BinaryExpression<'a>>,
    ) {
        if (binary_expression.operator == Equality || binary_expression.operator == StrictEquality)
            && ((binary_expression.right.is_identifier_reference()
                && binary_expression.left.is_call_expression())
                || (binary_expression.right.is_call_expression()
                    && binary_expression.left.is_identifier_reference()))
        {
            if let CallExpression(call_expression) = &binary_expression.right {
                if let Some(index) = binary_expression.left.get_identifier_reference() {
                    if index.name == self.pos && call_expression.arguments.len() == 1 {
                        self.handle_indexof_call_expression(call_expression);
                    }
                }
            } else if let CallExpression(call_expression) = &binary_expression.left {
                if let Some(index) = binary_expression.right.get_identifier_reference() {
                    if index.name == self.pos && call_expression.arguments.len() == 1 {
                        self.handle_indexof_call_expression(call_expression);
                    }
                }
            }
        }
    }

    /// Handle the call expression and check if it is a valid indexOf method call
    fn handle_indexof_call_expression<'a>(
        &mut self,
        call_expression: &'a oxc::allocator::Box<'a, oxc::ast::ast::CallExpression<'a>>,
    ) {
        if let Some(callee) = call_expression.callee.get_member_expr() {
            if let Some(static_property_name) = callee.static_property_name() {
                // check method call is indexOf and the object is the array identifier and the argument is the item identifier
                if static_property_name == "indexOf"
                    && (*get_function_target_identifier_name(call_expression)
                        == self.array_identifier
                        || *get_function_target_identifier_name(call_expression) == "self")
                    && self.has_valid_param(call_expression)
                {
                    // handle indexOf method call
                    self.matches.push((
                        String::from("pattern found"),
                        call_expression.callee.span().start,
                        call_expression.callee.span().end,
                    ));
                }
            }
        }
    }

    // endregion: handlers

    // region: helpers
    /// Check if the call expression has valid 'item' parameter
    fn has_valid_param<'a>(
        &mut self,
        call_expression: &oxc::allocator::Box<'a, oxc::ast::ast::CallExpression<'a>>,
    ) -> bool {
        if let Identifier(identifier) = call_expression.arguments[0].to_expression() {
            return identifier.name == self.item && call_expression.arguments.len() == 1;
        }
        false
    }

    /// Get the line and column of a given start position in the given input string
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

    /// Extract the binding identifiers from the function expression for item and pos for later matching
    fn extract_binding_identifiers_from_function_expression<'a>(
        &mut self,
        function_expression: &oxc::allocator::Box<'a, oxc::ast::ast::Function<'a>>,
    ) {
        if let BindingIdentifier(binding_identifier) =
            &function_expression.params.items[0].pattern.kind
        {
            self.item = binding_identifier.name.to_string();
        }
        if let BindingIdentifier(binding_identifier) =
            &function_expression.params.items[1].pattern.kind
        {
            self.pos = binding_identifier.name.to_string();
        }
    }

    /// Extract the binding identifiers from the arrow function expression for item and pos for later matching
    fn extract_binding_identifiers_from_arrow_function<'a>(
        &mut self,
        arrow_function_expression: &oxc::allocator::Box<
            'a,
            oxc::ast::ast::ArrowFunctionExpression<'a>,
        >,
    ) {
        if let BindingIdentifier(binding_identifier) =
            &arrow_function_expression.params.items[0].pattern.kind
        {
            self.item = binding_identifier.name.to_string();
        }
        if let BindingIdentifier(binding_identifier) =
            &arrow_function_expression.params.items[1].pattern.kind
        {
            self.pos = binding_identifier.name.to_string();
        }
    }
    // endregion: helpers
}

// region: helpers

/// Get the name (identifier) of the function target
fn get_function_target_identifier_name<'a>(
    call_expression: &'a oxc::ast::ast::CallExpression<'a>,
) -> &'a oxc::span::Atom {
    &call_expression
        .callee
        .get_inner_expression()
        .get_member_expr()
        .unwrap()
        .object()
        .get_identifier_reference()
        .unwrap()
        .name
}

/// Check if the arrow function expression has the correct number of parameters and is a non empty return statement
fn is_valid_return_statement<'a>(
    arrow_function_expression: &oxc::allocator::Box<'a, oxc::ast::ast::ArrowFunctionExpression<'a>>,
    function_body: &oxc::allocator::Box<'a, oxc::ast::ast::FunctionBody<'a>>,
) -> bool {
    arrow_function_expression.params.items.len() >= 2
        && !arrow_function_expression.expression
        && function_body.statements.len() > 0
}

/// Check if the arrow function expression has the correct number of parameters and is a non empty expression statement
fn is_valid_expression_statement<'a>(
    arrow_function_expression: &oxc::allocator::Box<'a, oxc::ast::ast::ArrowFunctionExpression<'a>>,
    function_body: &oxc::allocator::Box<'a, oxc::ast::ast::FunctionBody<'a>>,
) -> bool {
    arrow_function_expression.params.items.len() >= 2
        && arrow_function_expression.expression
        && function_body.statements.len() > 0
}
// endregion: helpers
// region: tests
#[cfg(test)]
mod tests {
    use super::*;
    use oxc::allocator::Allocator;

    use oxc::ast::Visit;
    use oxc::parser::Parser;
    use oxc::span::SourceType;

    /*
    let uniqueArray = array.filter((item, index) => array.indexOf(item) === index);
    let uniqueArray1 = array.filter(function(item, pos) { return array.indexOf(item) === pos });
    let uniqueArray2 = array.filter(function(item, pos, self) { return self.indexOf(item) === pos });
    let uniqueArray3 = array.filter((item, index, self) => self.indexOf(item) === index);
    let uniqueArray4 = array.filter((item, index) => index === array.indexOf(item));
    let uniqueArray5 = array.filter(function(item, pos) { return pos === array.indexOf(item) });
    let uniqueArray6 = array.filter(function(item, pos, self) { return pos === self.indexOf(item) });
    let uniqueArray7 = array.filter((item, index, self) => index === self.indexOf(item));
    let uniqueArray8 = array.filter((item, index) => array.indexOf(item) == index);
    let uniqueArray9 = array.filter(function(item, pos) { return array.indexOf(item) == pos });
    let uniqueArray10 = array.filter(function(item, pos, self) { return self.indexOf(item) == pos });
    let uniqueArray11 = array.filter((item, index, self) => self.indexOf(item) == index);
    let uniqueArray12 = array.filter((item, index) => index == array.indexOf(item));
    let uniqueArray13 = array.filter(function(item, pos) { return pos == array.indexOf(item) });
    let uniqueArray14 = array.filter(function(item, pos, self) { return pos == self.indexOf(item) });
    let uniqueArray15 = array.filter((item, index, self) => index == self.indexOf(item));

    let uniqueArray16 = array.filter((item, index) => {
    return array.indexOf(item) === index;
    });
    let uniqueArray17 = array.filter((item, index) => {
        return index === array.indexOf(item);
    });
    let uniqueArray18 = array.filter((item, index) => {
        return array.indexOf(item) == index;
    });
    let uniqueArray19 = array.filter((item, index) => {
        return index == array.indexOf(item);
    });
    let uniqueArray20 = array.filter((item, index, self) => {
        return self.indexOf(item) === index;
    });
    let uniqueArray21 = array.filter((item, index, self) => {
        return index === self.indexOf(item);
    });
    let uniqueArray22 = array.filter((item, index, self) => {
        return self.indexOf(item) == index;
    });
    let uniqueArray23 = array.filter((item, index, self) => {
        return index == self.indexOf(item);
    });
     */

    #[test]
    fn test_visit_program_arrow_two_params_strict_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray = array.filter((item, index) => array.indexOf(item) === index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_function_two_params_strict_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray1 = array.filter(function(item, pos) { return array.indexOf(item) === pos });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_function_three_params_strict_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray2 = array.filter(function(item, pos, self) { return self.indexOf(item) === pos });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_three_params_strict_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray3 = array.filter((item, index, self) => self.indexOf(item) === index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_two_params_strict_equality_reverse() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray4 = array.filter((item, index) => index === array.indexOf(item));";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_function_two_params_strict_equality_reverse() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray5 = array.filter(function(item, pos) { return pos === array.indexOf(item) });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_function_three_params_strict_equality_reverse() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray6 = array.filter(function(item, pos, self) { return pos === self.indexOf(item) });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_three_params_strict_equality_reverse() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray7 = array.filter((item, index, self) => index === self.indexOf(item));";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_two_params_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray8 = array.filter((item, index) => array.indexOf(item) == index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_function_two_params_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray9 = array.filter(function(item, pos) { return array.indexOf(item) == pos });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_function_three_params_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray10 = array.filter(function(item, pos, self) { return self.indexOf(item) == pos });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_three_params_equality_regular() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray11 = array.filter((item, index, self) => self.indexOf(item) == index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_two_params_equality_reverse() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray12 = array.filter((item, index) => index == array.indexOf(item));";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_function_two_params_equality_reverse() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray13 = array.filter(function(item, pos) { return pos == array.indexOf(item) });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_function_three_params_equality_reverse() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray14 = array.filter(function(item, pos, self) { return pos == self.indexOf(item) });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_three_params_equality_reverse() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray15 = array.filter((item, index, self) => index == self.indexOf(item));";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_two_params_strict_equality_regular_braces() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray16 = array.filter((item, index) => {
            return array.indexOf(item) === index;
            });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_two_params_strict_equality_reverse_braces() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray17 = array.filter((item, index) => {
            return index === array.indexOf(item);
        });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_two_params_equality_regular_braces() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray18 = array.filter((item, index) => {
            return array.indexOf(item) == index;
        });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };

        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_two_params_equality_reverse_braces() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray19 = array.filter((item, index) => {
            return index == array.indexOf(item);
        });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };

        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_three_params_strict_equality_regular_braces() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray20 = array.filter((item, index, self) => {
            return self.indexOf(item) === index;
        });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };

        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_three_params_strict_equality_reverse_braces() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray21 = array.filter((item, index, self) => {
            return index === self.indexOf(item);
        });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };

        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_three_params_equality_regular_braces() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray22 = array.filter((item, index, self) => {
            return self.indexOf(item) == index;
        });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };

        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }

    #[test]
    fn test_visit_program_arrow_three_params_equality_reverse_braces() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray23 = array.filter((item, index, self) => {
            return index == self.indexOf(item);
        });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };

        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 1);
        assert_eq!(ast_pass.matches[0].0, "pattern found");
    }
    // empty parameters
    #[test]
    fn test_visit_program_arrow_empty_params() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray15 = array.filter(() => { return index == self.indexOf(item) });";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 0);
    }

    #[test]
    fn test_visit_program_arrow_function_params() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray15 = array.filter(function() {index == self.indexOf(item)});";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };

        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 0);
    }

    // empty body
    #[test]
    fn test_visit_program_arrow_empty_body() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray15 = array.filter((item, index, self) => {});";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 0);
    }

    // empty body
    #[test]
    fn test_visit_program_function_empty_body() {
        let allocator = Allocator::default();
        let source_text = "let uniqueArray15 = array.filter(function(item, index, self) {} );";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 0);
    }

    // empty indexOf method call
    #[test]
    fn test_visit_program_empty_indexof_method_call() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray15 = array.filter((item, index, self) => self.indexOf() === index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 0);
    }

    // two params indexOf method call
    #[test]
    fn test_visit_program_two_params_indexof_method_call() {
        let allocator = Allocator::default();
        let source_text =
            "let uniqueArray15 = array.filter((item, index, self) => self.indexOf(item, startingIndex) === index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;
        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };

        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 0);
    }

    // all tests at once
    #[test]
    fn test_all_cases_at_once() {
        let allocator = Allocator::default();
        let source_text =
                "let uniqueArray = array.filter((item, index) => array.indexOf(item) === index);
                let uniqueArray1 = array.filter(function(item, pos) { return array.indexOf(item) === pos });
                let uniqueArray2 = array.filter(function(item, pos, self) { return self.indexOf(item) === pos });
                let uniqueArray3 = array.filter((item, index, self) => self.indexOf(item) === index);
                let uniqueArray4 = array.filter((item, index) => index === array.indexOf(item));
                let uniqueArray5 = array.filter(function(item, pos) { return pos === array.indexOf(item) });
                let uniqueArray6 = array.filter(function(item, pos, self) { return pos === self.indexOf(item) });
                let uniqueArray7 = array.filter((item, index, self) => index === self.indexOf(item));
                let uniqueArray8 = array.filter((item, index) => array.indexOf(item) == index);
                let uniqueArray9 = array.filter(function(item, pos) { return array.indexOf(item) == pos });
                let uniqueArray10 = array.filter(function(item, pos, self) { return self.indexOf(item) == pos });
                let uniqueArray11 = array.filter((item, index, self) => self.indexOf(item) == index);
                let uniqueArray12 = array.filter((item, index) => index == array.indexOf(item));
                let uniqueArray13 = array.filter(function(item, pos) { return pos == array.indexOf(item) });
                let uniqueArray14 = array.filter(function(item, pos, self) { return pos == self.indexOf(item) });
                let uniqueArray15 = array.filter((item, index, self) => index == self.indexOf(item));
                let uniqueArray16 = array.filter((item, index) => {return array.indexOf(item) === index;});
                let uniqueArray17 = array.filter((item, index) => {return index === array.indexOf(item);});
                let uniqueArray18 = array.filter((item, index) => {return array.indexOf(item) == index;});
                let uniqueArray19 = array.filter((item, index) => {return index == array.indexOf(item);});
                let uniqueArray20 = array.filter((item, index, self) => {return self.indexOf(item) === index;});
                let uniqueArray21 = array.filter((item, index, self) => {return index === self.indexOf(item);});
                let uniqueArray22 = array.filter((item, index, self) => {return self.indexOf(item) == index;});
                let uniqueArray23 = array.filter((item, index, self) => {return index == self.indexOf(item);});
                let uniqueArray24 = array.filter(() => {});
                let uniqueArray100 = array.filter(function(item, index, self) {} );
                let uniqueArray110 = array.filter(function() { });
                let uniqueArray230 = array.filter((item, index, self) => {return index == self.indexOf(self);});
                let uniqueArray150 = array.filter((item, index, self) => self.indexOf() === index);
                let uniqueArray250 = array.filter((item, index, self) => self.indexOf(item, startingIndex) === index);";
        let source_type = SourceType::from_path("javscript.js").unwrap();
        let ret = Parser::new(&allocator, source_text, source_type).parse();
        let program = ret.program;

        let mut ast_pass = Duplicates {
            matches: vec![],
            array_identifier: String::from(""),
            item: String::from(""),
            pos: String::from(""),
        };
        ast_pass.visit_program(&program);
        assert_eq!(ast_pass.matches.len(), 24);
    }
}
// endregion: tests

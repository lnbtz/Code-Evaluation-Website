use oxc::ast::ast::Program;
pub struct Ctx<'a> {
    pub input: &'a str,
    pub program: &'a Program<'a>,
}

impl<'a> Ctx<'a> {
    pub fn new(input: &'a str, program: &'a Program<'a>) -> Self {
        Self { input, program }
    }
}

pub struct JavsScriptCtx<'a> {
    pub input: &'a str,
    pub program: &'a Program<'a>,
}

impl<'a> JavsScriptCtx<'a> {
    pub fn new(input: &'a str, program: &'a Program<'a>) -> Self {
        Self { input, program }
    }
}

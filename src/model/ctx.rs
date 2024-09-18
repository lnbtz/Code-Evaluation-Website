use oxc::ast::ast::Program;
pub enum Ctx<'a> {
    JavaScriptCtx(JavaScriptCtx<'a>),
    CssCtx(CssCtx<'a>),
    HtmlCtx(HtmlCtx<'a>),
}

pub struct JavaScriptCtx<'a> {
    pub input: &'a str,
    pub program: &'a Program<'a>,
}

pub struct CssCtx<'a> {
    pub input: &'a str,
}

pub struct HtmlCtx<'a> {
    pub input: &'a str,
}

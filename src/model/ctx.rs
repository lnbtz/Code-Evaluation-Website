use oxc::ast::ast::Program;
pub struct Ctx<'a> {
    pub java_script_ctx: &'a Option<JavaScriptCtx<'a>>,
    pub css_ctx: &'a Option<CssCtx<'a>>,
    pub html_ctx: &'a Option<HtmlCtx<'a>>,
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

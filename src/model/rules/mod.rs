use self::css::{dry::Dry, minify::Minify};

pub mod css;
pub mod html;
pub mod js;

// rules interface here with one function that returns a line result or nothing
pub trait Rule {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn apply(&self, input: &str) -> Option<Vec<LineResult>>;
}
// also uses these structs to build the line result
#[derive(Clone)]
pub struct LineResult {
    pub severity: Severity,
    pub line: i32,
    pub column: i32,
    pub classification: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Warning,
    Info,
}
impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn load_rules(file_typle: String, rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    match file_typle.as_str() {
        "css" => load_css_rules(rules_to_load),
        "html" => load_html_rules(rules_to_load),
        "js" => load_js_rules(rules_to_load),
        "java" => load_java_rules(rules_to_load),
        _ => panic!("Unknown file type: {}", file_typle),
    }
}

pub fn load_css_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return vec![
            Box::new(Dry) as Box<dyn Rule>,
            Box::new(Minify) as Box<dyn Rule>,
        ];
    }
    // TODO think of smart way to statically create rules and then load them when needed
    rules_to_load
        .iter()
        .map(|rule| match rule.as_str() {
            "css-dry" => Box::new(Dry) as Box<dyn Rule>,
            "css-minify" => Box::new(Minify) as Box<dyn Rule>,
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}

pub fn load_html_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return vec![Box::new(html::loading::Loading) as Box<dyn Rule>];
    }
    // TODO think of smart way to statically create rules and then load them when needed
    rules_to_load
        .iter()
        .map(|rule| match rule.as_str() {
            "lazy-loading" => Box::new(html::loading::Loading) as Box<dyn Rule>,
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}

pub fn load_js_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return vec![Box::new(js::minify::Minify) as Box<dyn Rule>];
    }
    // TODO think of smart way to statically create rules and then load them when needed
    rules_to_load
        .iter()
        .map(|rule| match rule.as_str() {
            "js-minify" => Box::new(js::minify::Minify) as Box<dyn Rule>,
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}

fn load_java_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return vec![];
    }
    // TODO think of smart way to statically create rules and then load them when needed
    rules_to_load
        .iter()
        .map(|rule| match rule.as_str() {
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}

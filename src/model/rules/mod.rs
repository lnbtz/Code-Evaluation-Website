use self::css::minify::Minify;

pub mod css;
pub mod html;
pub mod js;

/// Rule trait that all rules must implement
pub trait Rule {
    /// get the name of the rule
    fn get_name(&self) -> &str;
    /// get the description of the rule
    fn get_description(&self) -> &str;
    /// apply the rule to the input and return the results
    fn apply(&self, input: &str) -> Option<Vec<LineResult>>;
}

/// LineResult is a struct that holds the result of a rule applied to a line
#[derive(Clone)]
pub struct LineResult {
    pub severity: Severity,
    pub line: i32,
    pub column: i32,
    pub classification: String,
    pub description: String,
}

/// Severity is an enum that holds the severity of a rule
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

/// load_rules loads the rules based on the file type and the rules to load
pub fn load_rules(file_type: String, rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    match file_type.as_str() {
        "css" => load_css_rules(rules_to_load),
        "html" => load_html_rules(rules_to_load),
        "js" => load_js_rules(rules_to_load),
        "java" => load_java_rules(rules_to_load),
        _ => panic!("Unknown file type: {}", file_type),
    }
}

/// load_css_rules loads the css rules based on the rules to load
pub fn load_css_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return vec![Box::new(Minify) as Box<dyn Rule>];
    }
    rules_to_load
        .iter()
        .map(|rule| match rule.as_str() {
            "css-minify" => Box::new(Minify) as Box<dyn Rule>,
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}

/// load_html_rules loads the html rules based on the rules to load
pub fn load_html_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return vec![Box::new(html::loading::Loading) as Box<dyn Rule>];
    }
    rules_to_load
        .iter()
        .map(|rule| match rule.as_str() {
            "lazy-loading" => Box::new(html::loading::Loading) as Box<dyn Rule>,
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}

/// load_js_rules loads the js rules based on the rules to load
pub fn load_js_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return vec![
            Box::new(js::minify::Minify) as Box<dyn Rule>,
            Box::new(js::methods::Methods::default()) as Box<dyn Rule>,
            Box::new(js::duplicates::Duplicates::default()) as Box<dyn Rule>,
        ];
    }
    rules_to_load
        .iter()
        .map(|rule| match rule.as_str() {
            "js-minify" => Box::new(js::minify::Minify) as Box<dyn Rule>,
            "js-method-calls" => Box::new(js::methods::Methods::default()) as Box<dyn Rule>,
            "js-duplicates" => Box::new(js::duplicates::Duplicates::default()) as Box<dyn Rule>,
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}

/// load_java_rules loads the java rules based on the rules to load
fn load_java_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return vec![];
    }
    rules_to_load
        .iter()
        .map(|rule| match rule.as_str() {
            "some-rule" => panic!("Not implemented yet"),
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}

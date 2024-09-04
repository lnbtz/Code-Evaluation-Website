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
/// if new programming langueages are added, they should be added here as a new match case
/// with the corresponding load function
pub fn load_rules(file_type: String, rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    match file_type.as_str() {
        "css" => load_css_rules(rules_to_load),
        "html" => load_html_rules(rules_to_load),
        "js" => load_js_rules(rules_to_load),
        _ => panic!("Unknown file type: {}", file_type),
    }
}

/// load_css_rules loads the css rules based on the rules to load
/// new rules have to be added to the match case
pub fn load_css_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    let rules = vec![Box::new(Minify) as Box<dyn Rule>];
    filter_rules(rules_to_load, rules)
}

/// load_html_rules loads the html rules based on the rules to load
/// new rules have to be added to the match case
pub fn load_html_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    let rules = vec![Box::new(html::loading::Loading) as Box<dyn Rule>];
    filter_rules(rules_to_load, rules)
}

/// load_js_rules loads the js rules based on the rules to load
/// new rules have to be added to the match case
pub fn load_js_rules(rules_to_load: Vec<String>) -> Vec<Box<dyn Rule>> {
    let rules = vec![
        Box::new(js::minify::Minify) as Box<dyn Rule>,
        Box::new(js::duplicates::Duplicates::default()) as Box<dyn Rule>,
        // Box::new(js::template_rule::TemplateRule) as Box<dyn Rule>,
    ];
    filter_rules(rules_to_load, rules)
}

fn filter_rules(rules_to_load: Vec<String>, rules: Vec<Box<dyn Rule>>) -> Vec<Box<dyn Rule>> {
    if rules_to_load.is_empty() {
        return rules;
    }
    rules
        .into_iter()
        .filter(|rule| rules_to_load.contains(&rule.get_name().to_lowercase()))
        .collect()
}

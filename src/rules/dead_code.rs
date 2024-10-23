use crate::linter::Issue;
use regex::Regex;

pub fn check(content: &str) -> Vec<Issue> {
    let mut issues = Vec::new();
    let re = Regex::new(r"fn\s+(\w+)\s*\(").expect("Failed to create regex");
    for (i, cap) in re.captures_iter(content).enumerate() {
        let fn_name = &cap[1];
        if !content.contains(&format!("{}(", fn_name)) {
            issues.push(Issue {
                line: i + 1,
                message: format!(
                    "Dead code detected. Function '{}' is defined but never used.",
                    fn_name
                ),
            });
        }
    }
    issues
}

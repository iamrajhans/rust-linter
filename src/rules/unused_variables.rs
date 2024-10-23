use crate::linter::Issue;
use regex::Regex;

pub fn check(content: &str) -> Vec<Issue> {
    let mut issues = Vec::new();
    let re = Regex::new(r"let\s+(\w+):?\s*\w*\s*=\s*.*;").expect("Failed to create regex");
    for (i, cap) in re.captures_iter(content).enumerate() {
        let var_name = &cap[1];
        if !content.contains(var_name) {
            issues.push(Issue {
                line: i + 1,
                message: format!("Unused variable '{}' detected.", var_name),
            });
        }
    }
    issues
}

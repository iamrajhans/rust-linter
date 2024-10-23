use crate::linter::Issue;

pub fn check(content: &str) -> Vec<Issue> {
    let mut issues = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("for ") || line.contains("while ") {
            issues.push(Issue {
                line: i + 1,
                message: String::from("Loop detected. Consider checking its complexity."),
            });
        }
        if line.contains("if ") && line.matches("if").count() > 2 {
            issues.push(Issue {
                line: i + 1,
                message: String::from("Nested if statements detected. Consider simplifying."),
            });
        }
    }
    issues
}

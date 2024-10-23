use crate::linter::Issue;

pub fn check(content: &str) -> Vec<Issue> {
    let mut issues = Vec::new();

    // Iterate over the content line by line
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        // Skip empty lines and lines that are comments
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
            continue;
        }

        // Check if the line contains the 'fn' keyword
        if let Some(fn_index) = trimmed_line.find("fn ") {
            // Ensure 'fn' is a standalone keyword
            let before_fn = &trimmed_line[..fn_index];
            if !before_fn.chars().all(|c| c.is_whitespace()) {
                continue; // 'fn' is not a standalone keyword
            }

            // Extract the function name
            let after_fn = &trimmed_line[fn_index + 3..]; // Skip 'fn '
            let function_name = after_fn
                .split(|c: char| c == '(' || c.is_whitespace())
                .next()
                .unwrap_or("");

            // Check if the function name is not empty and not snake_case
            if !function_name.is_empty() && !is_snake_case(function_name) {
                issues.push(Issue {
                    line: i + 1,
                    message: format!(
                        "Function '{}' does not follow snake_case naming convention.",
                        function_name
                    ),
                });
            }
        }
    }

    issues
}

fn is_snake_case(name: &str) -> bool {
    // Name should not be empty and should start with a lowercase letter or underscore
    if name.is_empty()
        || (!name.chars().next().unwrap().is_lowercase() && name.chars().next().unwrap() != '_')
    {
        return false;
    }

    // Check each character
    let mut prev_char = '\0';
    for c in name.chars() {
        if c == '_' {
            if prev_char == '_' {
                return false; // No consecutive underscores
            }
        } else if !c.is_lowercase() && !c.is_numeric() {
            return false; // Only lowercase letters and numbers allowed
        }
        prev_char = c;
    }

    // Name should not end with an underscore
    if name.ends_with('_') {
        return false;
    }

    true
}

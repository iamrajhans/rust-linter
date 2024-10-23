use crate::rules;
use rayon::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Issue {
    pub line: usize,
    pub message: String,
}

pub fn analyze(content: &str) -> Vec<Issue> {
    let rules: Vec<fn(&str) -> Vec<Issue>> = vec![
        rules::unused_variables::check,
        rules::naming_conventions::check,
        rules::complexity::check,
        rules::dead_code::check,
    ];

    rules.par_iter().flat_map(|rule| rule(content)).collect()
}

pub fn report_issues(issues: Vec<Issue>, output_format: &str, file_path: &str) {
    match output_format {
        "json" => {
            let json_output = serde_json::to_string_pretty(&issues).unwrap();
            println!("{}", json_output);
        }
        _ => {
            for issue in issues {
                println!(
                    "File: {}, Line {}: {}",
                    file_path, issue.line, issue.message
                );
            }
        }
    }
}

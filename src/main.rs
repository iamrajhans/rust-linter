mod linter;
use clap::{Arg, Command};
use log::{error, info};
use std::error::Error;
use std::fs;
use std::fs::read_dir;
use std::path::Path;
use std::process::Command as ProcessCommand;
pub mod rules;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Code Linter")
        .version("0.1.0")
        .author("Rajhans")
        .about("Lints Rust code for common issues")
        .arg(
            Arg::new("path")
                .help("The Rust file or directory to lint")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Specify the output format (plain, json)")
                .short('o')
                .long("output")
                .required(false)
                .default_value("plain"),
        )
        .arg(
            Arg::new("fix")
                .help("Automatically fix linting issues using rustfmt")
                .short('f')
                .long("fix")
                .required(false)
                .action(clap::ArgAction::SetFalse),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let output_format = matches.get_one::<String>("output").unwrap();
    let fix = matches.get_flag("fix");

    if Path::new(path).is_file() {
        lint_file(path, output_format, fix)?;
    } else if Path::new(path).is_dir() {
        lint_directory(path, output_format, fix)?;
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Specified path is neither a file nor a directory",
        )));
    }

    Ok(())
}

fn lint_directory(dir_path: &str, output_format: &str, fix: bool) -> Result<(), Box<dyn Error>> {
    for entry in read_dir(dir_path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_dir() {
            lint_directory(file_path.to_str().unwrap(), output_format, fix)?;
        } else if file_path.extension().map_or(false, |ext| ext == "rs") {
            lint_file(file_path.to_str().unwrap(), output_format, fix)?;
        }
    }
    Ok(())
}

fn lint_file(file_path: &str, output_format: &str, fix: bool) -> Result<(), Box<dyn Error>> {
    if fix {
        match ProcessCommand::new("rustfmt").arg(file_path).output() {
            Ok(output) => {
                if !output.status.success() {
                    error!(
                        "Failed to format file: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to run rustfmt",
                    )));
                } else {
                    info!("Successfully formatted the file using rustfmt.");
                }
            }
            Err(e) => {
                error!("Failed to execute rustfmt: {}", e);
                return Err(Box::new(e));
            }
        }
    }

    match fs::read_to_string(file_path) {
        Ok(content) => {
            let issues = linter::analyze(&content);
            linter::report_issues(issues, output_format, file_path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to read file: {}", e);
            Err(Box::new(e))
        }
    }
}

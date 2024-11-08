use antex::{ColorMode, StyledText, Text};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const EXCLUDED_NAMES: [&str; 1] = ["target"];

const ACCEPTED_EXTENSIONS: [&str; 7] = ["rs", "toml", "md", "html", "txt", "js", "ts"];

const GUTTER: usize = 100;

fn visit<N, E, A>(root: &str, excluded_name: N, accepted_extension: E, action: A)
where
    N: Fn(&str) -> bool,
    E: Fn(&str) -> bool,
    A: Fn(&Path),
{
    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        let file_type = entry.file_type();
        if file_type.is_file() {
            let skip = entry.path().iter().any(|segment| {
                let name = segment.to_string_lossy().to_string();
                (name.len() > 1 && name.starts_with(".")) || excluded_name(&name)
            });
            if !skip {
                if let Some(extension) = entry.path().extension() {
                    let extension = extension.to_string_lossy().to_string();
                    if accepted_extension(&extension) {
                        action(entry.path());
                    }
                }
            }
        }
    }
}

fn main() {
    let cm = ColorMode::default();
    let is_excluded_name = |name: &str| {
        //
        EXCLUDED_NAMES.contains(&name)
    };
    let is_accepted_extension = |extension: &str| {
        //
        ACCEPTED_EXTENSIONS.contains(&extension)
    };
    let action = |path: &Path| {
        let content = fs::read_to_string(path).unwrap();
        let path_name = path.display().to_string();
        let ruler = ".".repeat(GUTTER.saturating_sub(path_name.len()));
        if content.ends_with("\n\n") {
            Text::new(cm)
                .red()
                .s("ERR")
                .clear()
                .space()
                .blue()
                .s(path_name)
                .space()
                .clear()
                .s(ruler)
                .space()
                .yellow()
                .s("ends with multiple newlines.")
                .cprintln();
        } else if content.ends_with("\n") {
            //println!(" OK {}", path.display());
        } else {
            Text::new(cm)
                .red()
                .s("ERR")
                .clear()
                .space()
                .blue()
                .s(path_name)
                .space()
                .clear()
                .s(ruler)
                .space()
                .red()
                .s("is missing the trailing newline.")
                .cprintln();
        }
    };
    visit(".", is_excluded_name, is_accepted_extension, action);
}

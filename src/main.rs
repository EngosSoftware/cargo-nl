use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const EXCLUDED_NAMES: [&str; 1] = ["target"];
const ACCEPTED_EXTENSIONS: [&str; 4] = ["rs", "toml", "md", "html"];

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
        if content.ends_with("\n\n") {
            println!("ERR {} ends with multiple newlines.", path.display());
        } else if content.ends_with("\n") {
            println!(" OK {}", path.display());
        } else {
            println!("ERR {} is missing the trailing newline.", path.display());
        }
    };
    visit(".", is_excluded_name, is_accepted_extension, action);
}

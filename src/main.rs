use clap::Parser;
use dirs::DEFAULT_TEMPLATES_DIR;
use include_dir::{Dir, DirEntry};
use std::{fs, io, path::Path};
mod dirs;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Name of the new project
    #[clap(short, long)]
    name: Option<String>,
}

fn copy_and_replace(
    source_dir: &Dir,
    destination_path: &Path,
    project_name: &str,
) -> io::Result<()> {
    const WORD_TO_REPLACE: &str = "PROJECT_NAME";
    fs::create_dir_all(destination_path)?;

    for entry in source_dir.entries() {
        let relative_path = entry.path().strip_prefix(source_dir.path()).unwrap();
        let entry_path = &destination_path.join(relative_path);
        match entry {
            DirEntry::Dir(dir) => {
                copy_and_replace(&dir, &entry_path, project_name)?;
            }
            DirEntry::File(file) => {
                if let Some(content) = file.contents_utf8() {
                    let new_content = content.replace(WORD_TO_REPLACE, project_name);
                    fs::write(&entry_path, new_content)?;
                }
            }
        }
    }

    Ok(())
}

/// Create a new gpui app
fn main() -> io::Result<()> {
    let args = Args::parse();

    let project_name = args.name.unwrap_or_else(|| "gpui-app".to_string());
    let project_path = Path::new(&project_name);

    if project_path.exists() {
        println!("'{}' already exists.", project_name);
        return Ok(());
    }

    copy_and_replace(&DEFAULT_TEMPLATES_DIR, project_path, &project_name)?;

    println!(
        "Successfully created new gpui app project '{}'",
        project_name
    );

    Ok(())
}

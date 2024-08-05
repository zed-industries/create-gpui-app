use clap::Parser;
use include_dir::{include_dir, Dir, DirEntry};
use std::{fs, io, path::Path};

static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Name of the new project
    #[clap(short, long, default_value = "gpui-app")]
    name: Option<String>,
    /// Setup your project as a workspace
    #[clap(short, long)]
    workspace: bool,
}

fn copy_and_replace(
    source_dir: &Dir,
    destination_path: &Path,
    project_name: &str,
) -> io::Result<()> {
    const WORD_TO_REPLACE: &str = "PROJECT_NAME";
    let new_destination_path = if destination_path.file_name().unwrap() == WORD_TO_REPLACE {
        destination_path.with_file_name(project_name)
    } else {
        destination_path.to_owned()
    };
    fs::create_dir_all(&new_destination_path)?;

    for entry in source_dir.entries() {
        let relative_path = entry.path().strip_prefix(source_dir.path()).unwrap();
        let entry_path = new_destination_path.join(relative_path);
        match entry {
            DirEntry::Dir(dir) => copy_and_replace(&dir, &entry_path, project_name)?,
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

    let project_name = args.name.unwrap();
    let project_path = Path::new(&project_name);
    let is_workspace = args.workspace;

    if project_path.exists() {
        println!("'{}' already exists.", project_name);
        return Ok(());
    }

    copy_and_replace(
        if is_workspace {
            TEMPLATES_DIR.get_dir("src/templates/workspace").unwrap()
        } else {
            TEMPLATES_DIR.get_dir("src/templates/default").unwrap()
        },
        project_path,
        &project_name,
    )?;

    println!(
        "Successfully created new gpui app project '{}'",
        project_name
    );

    Ok(())
}

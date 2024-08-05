use clap::Parser;
use include_dir::{include_dir, Dir, DirEntry};
use std::{fs, io, path::Path};

static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Name of the new project
    #[clap(short, long, default_value = "gpui-app")]
    name: Option<String>,
    /// Setup your project as a workspace
    #[clap(short, long)]
    workspace: bool,
    /// Setup a zed style project
    #[clap(short, long)]
    zed: bool,
}

fn copy_and_replace(
    source_dir: &Dir,
    destination_path: &Path,
    project_name: &str,
    is_zed_style: bool,
    is_workspace: bool,
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
            DirEntry::Dir(dir) => {
                copy_and_replace(&dir, &entry_path, project_name, is_zed_style, is_workspace)?
            }
            DirEntry::File(file) => {
                if let Some(content) = file.contents_utf8() {
                    let new_content = content.replace(WORD_TO_REPLACE, project_name);
                    let mut new_entry_path = if file.path().file_name().unwrap() == "_Cargo.toml" {
                        entry_path.with_file_name("Cargo.toml")
                    } else {
                        entry_path.to_owned()
                    };
                    fs::write(&new_entry_path, new_content)?;
                    let mut content = content.to_string();
                    if is_zed_style {
                        match file.path().as_os_str().to_str().unwrap() {
                            path if path.ends_with("main.rs") => {
                                new_entry_path.set_file_name(format!("{}.rs", project_name));
                            }
                            path if path.ends_with("Cargo.toml") => {
                                if is_workspace {
                                    if path.contains("crates") {
                                        content = content.replace(
                                            "src/main.rs",
                                            format!("src/{}.rs", project_name).as_str(),
                                        );
                                    }
                                } else {
                                    let additional_content = "\n[[bin]]\nname = \"PROJECT_NAME\"\npath = \"src/PROJECT_NAME.rs\"";
                                    content.push_str(additional_content);
                                }
                            }
                            _ => {}
                        }
                    }
                    content = content.replace(WORD_TO_REPLACE, project_name);
                    fs::write(&new_entry_path, content)?;
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
    let is_zed_style = args.zed;

    if project_path.exists() {
        println!("'{}' already exists.", project_name);
        return Ok(());
    }

    copy_and_replace(
        if is_workspace {
            TEMPLATES_DIR.get_dir("workspace").unwrap()
        } else {
            TEMPLATES_DIR.get_dir("default").unwrap()
        },
        project_path,
        &project_name,
        is_zed_style,
        is_workspace,
    )?;

    println!(
        "Successfully created new gpui app project '{}'",
        project_name
    );

    Ok(())
}

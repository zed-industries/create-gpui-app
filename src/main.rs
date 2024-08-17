use clap::Parser;
use include_dir::{include_dir, Dir, DirEntry};
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
};

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
    destination_path: &mut PathBuf,
    is_workspace: bool,
    is_zed_styled: bool,
    project_name: &str,
    source_dir: &Dir,
) -> Result<()> {
    const WORD_TO_REPLACE: &str = "PROJECT_NAME";
    if destination_path.file_name().unwrap() == WORD_TO_REPLACE {
        destination_path.set_file_name(project_name)
    };

    fs::create_dir_all(&destination_path)?;
    for entry in source_dir.entries() {
        let relative_path = entry.path().strip_prefix(source_dir.path()).unwrap();
        let mut entry_path = destination_path.to_owned().join(relative_path);
        match entry {
            DirEntry::Dir(dir) => copy_and_replace(
                &mut entry_path,
                is_workspace,
                is_zed_styled,
                project_name,
                &dir,
            )?,
            DirEntry::File(file) => {
                if let Some(content) = file.contents_utf8() {
                    let mut content_string = content.to_string();
                    match file.path() {
                        path if path.file_name().unwrap() == "main.rs" => {
                            if is_zed_styled {
                                entry_path.set_file_name(format!("{}.rs", project_name))
                            };
                        }
                        path if path.file_name().unwrap() == "_Cargo.toml" => {
                            if is_zed_styled {
                                let additional_content = "\n[[bin]]\nname = \"PROJECT_NAME\"\npath = \"src/PROJECT_NAME.rs\"";
                                if is_workspace {
                                    if path
                                        .components()
                                        .any(|component| component.as_os_str() == "crates")
                                    {
                                        content_string = content_string.replace(
                                            "src/main.rs",
                                            format!("src/{}.rs", project_name).as_str(),
                                        );
                                    }
                                } else {
                                    content_string.push_str(additional_content);
                                }
                            }

                            entry_path.set_file_name("Cargo.toml")
                        }
                        _ => {}
                    }
                    content_string = content_string.replace(WORD_TO_REPLACE, project_name);
                    fs::write(&entry_path, content_string)?;
                }
            }
        }
    }
    Ok(())
}

/// Create a new gpui app
fn main() -> Result<()> {
    let args = Args::parse();

    let project_name = args.name.unwrap();
    let mut project_path = Path::new(&project_name).to_owned();
    let is_workspace = args.workspace;
    let is_zed_styled = args.zed;

    if project_path.exists() {
        println!("'{}' already exists.", project_name);
        return Ok(());
    }

    copy_and_replace(
        &mut project_path,
        is_workspace,
        is_zed_styled,
        &project_name,
        if is_workspace {
            TEMPLATES_DIR.get_dir("workspace").unwrap()
        } else {
            TEMPLATES_DIR.get_dir("default").unwrap()
        },
    )?;

    println!(
        "Successfully created new gpui app project '{}'",
        project_name
    );

    Ok(())
}

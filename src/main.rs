use clap::Parser;
use std::{
    fs,
    io::{self, Read},
    path::Path,
};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Name of the new project
    #[clap(short, long)]
    name: Option<String>,
}

fn copy_and_replace(source: &Path, destination: &Path, project_name: &str) -> io::Result<()> {
    const WORD_TO_REPLACE: &str = "PROJECT_NAME";

    if source.is_dir() {
        fs::create_dir_all(destination)?;

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let relative_path = entry_path.strip_prefix(source).unwrap();
            let dest_path = destination.join(relative_path);

            copy_and_replace(&entry_path, &dest_path, project_name)?;
        }
    } else if source.is_file() {
        let mut content = String::new();
        fs::File::open(source)?.read_to_string(&mut content)?;

        let new_content = content.replace(WORD_TO_REPLACE, project_name);
        fs::write(destination, new_content)?;
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

    copy_and_replace(
        Path::new("src/templates/default"),
        project_path,
        &project_name,
    )?;

    println!(
        "Successfully created new gpui app project '{}'",
        project_name
    );

    Ok(())
}

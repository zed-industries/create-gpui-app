use clap::Parser;
use fs_extra::dir::create_all;
use std::fs::{self, create_dir};

mod templates;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Name of the new project
    #[clap(short, long)]
    name: Option<String>,
}

/// Create a new gpui app
fn main() {
    let args = Args::parse();

    let project_name = &args.name.unwrap_or("gpui_app".to_string());
    let project_path = format!("./{}", project_name);
    let src_path = format!("{}/src", project_path);

    if fs::metadata(&project_path).is_ok() {
        println!("'{}' already exists.", project_name);
        return;
    } else {
        create_all(&project_path, false).expect("Failed to create project directory");
        create_dir(&src_path).expect("Failed to create src directory");
    }

    let readme_content = templates::default::readme(project_name);
    let cargo_toml_content = templates::default::create_cargo_toml(project_name);
    let main_rs_content = templates::default::create_main();

    let cargo_toml_path = format!("{}/Cargo.toml", project_path);
    let main_rs_path = format!("{}/main.rs", src_path);

    fs::write(format!("{}/README.md", project_path), readme_content)
        .expect("Failed to write README.md");
    fs::write(cargo_toml_path, cargo_toml_content).expect("Failed to write Cargo.toml");
    fs::write(main_rs_path, main_rs_content).expect("Failed to write main.rs");

    println!(
        "Successfully created new gpui app project '{}'",
        project_name
    );
}

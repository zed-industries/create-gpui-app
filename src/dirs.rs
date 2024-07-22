use include_dir::{include_dir, Dir};

pub static DEFAULT_TEMPLATES_DIR: Dir<'_> = include_dir!("src/templates/default");

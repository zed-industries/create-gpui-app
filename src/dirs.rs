use include_dir::{include_dir, Dir};

pub static DEFAULT_TEMPLATE_DIR: Dir<'_> = include_dir!("src/templates/default");
pub static WORKSPACE_TEMPLATE_DIR: Dir<'_> = include_dir!("src/templates/workspace");

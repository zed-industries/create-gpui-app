use include_dir::{include_dir, Dir};

pub static DEFAULT_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/src/templates/default");
pub static WORKSPACE_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/src/templates/workspace");

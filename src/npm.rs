use crate::{registry::Registry, buffer::Lang};

pub struct Npm {}

// latest version of a package
// npm view prettier version

impl Registry for Npm {
    fn can_handle(file: &str) -> bool {
        file == "package.json"
    }

    fn cmd_latest_version(pkg: &str) -> String {
        format!("npm view {} version", pkg)
    }

    fn manifest_type() -> Lang {
        Lang::Json
    }
}

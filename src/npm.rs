use std::path::Path;

use crate::{buffer::Lang, proc::Cmd, registry::Registry};

pub struct Npm;

impl Registry for Npm {
    fn can_handle(&self, file: &Path) -> bool {
        file.ends_with("package.json")
    }

    fn latest_version(&self, pkg: &str) -> Cmd {
        Cmd::new("npm", ["view", pkg, "version"])
    }

    fn latest_version_parse(&self, out: &str) -> String {
        out.trim_end().to_owned()
    }

    fn manifest_type(&self) -> Lang {
        Lang::Json
    }
}

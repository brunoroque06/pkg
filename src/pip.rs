use std::path::Path;

use crate::{buffer::Lang, proc::Cmd, registry::Registry};

pub struct Pip;

impl Registry for Pip {
    fn can_handle(&self, file: &Path) -> bool {
        file.ends_with("pyproject.toml")
    }

    fn latest_version(&self, pkg: &str) -> Cmd {
        Cmd::new("pip3", ["index", "version", "--json", pkg])
    }

    fn latest_version_parse(&self, out: &str) -> String {
        todo!()
    }

    fn manifest_type(&self) -> Lang {
        Lang::Toml
    }
}

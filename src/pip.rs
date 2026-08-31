use crate::{buffer::Lang, proc::Cmd, registry::Registry};

struct Pip {}

impl Registry for Pip {
    fn can_handle(file: &str) -> bool {
        file.ends_with("pyproject.toml")
    }

    fn latest_version(pkg: &str) -> Cmd {
        Cmd::new("pip3", ["index", "version", "--json", pkg])
    }

    fn latest_version_parse(out: &str) -> String {
        todo!()
    }

    fn manifest_type() -> Lang {
        Lang::Toml
    }
}

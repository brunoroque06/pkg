use std::path::Path;

use crate::{buffer::Lang, proc::Cmd, registry::Registry};

pub struct Pip;

impl Registry for Pip {
    fn cmd_parse(&self, out: &str) -> Result<String, String> {
        Ok(out.to_owned())
    }

    fn cmd_version(&self, pkg: &str) -> Cmd {
        Cmd::new("pip3", ["index", "version", "--json", pkg])
    }

    fn deps_query(&self) -> String {
        todo!()
    }

    fn manifest(&self) -> Lang {
        Lang::Toml
    }

    fn supports(&self, file: &Path) -> bool {
        file.ends_with("pyproject.toml")
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, pip::Pip, registry::Registry};

    const TOML: &str = include_str!("../tests/manifests/pyproject.toml");

    // #[test]
    // fn query() {
    //     let buf = Buffer::new(TOML.to_owned(), Pip.manifest_type()).expect("should parse");
    //     let pairs = buf.query_pairs(&Pip.deps_query()).expect("should query");
    //     assert_eq!(pairs.len(), 3);
    // }
}

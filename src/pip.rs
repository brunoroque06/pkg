use std::path::Path;

use crate::{buffer::Lang, proc::Cmd, registry::Registry};

pub struct Pip;

impl Registry for Pip {
    fn manifest(&self) -> Lang {
        Lang::Toml
    }

    fn parse_version(&self, out: &str) -> Result<String, String> {
        Ok(out.to_owned())
    }

    fn query_deps(&self) -> String {
        todo!()
    }

    fn supports(&self, file: &Path) -> bool {
        file.ends_with("pyproject.toml")
    }

    fn version(&self, pkg: &str) -> Cmd {
        Cmd::new("pip3", ["index", "version", "--json", pkg])
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, pip::Pip, registry::Registry};

    const TOML: &str = r#"
        [project]
        dependencies = [
          "lib==1.2.3",
        ]

        [dependency-groups]
        dev = [
          "lib2==1.0.0",
          "lib3==9.0.3",
        ]
    "#;

    // #[test]
    // fn query() {
    //     let buf = Buffer::new(TOML.to_owned(), Pip.manifest_type()).expect("should parse");
    //     let pairs = buf.query_pairs(&Pip.deps_query()).expect("should query");
    //     assert_eq!(pairs.len(), 3);
    // }
}

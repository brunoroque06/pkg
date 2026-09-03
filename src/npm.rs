use std::path::Path;

use crate::{buffer::Lang, proc::Cmd, registry::Registry};

pub struct Npm;

impl Registry for Npm {
    fn manifest(&self) -> Lang {
        Lang::Json
    }

    fn parse_version(&self, out: &str) -> Result<String, String> {
        Ok(out.trim_end().to_owned())
    }

    fn query_deps(&self) -> String {
        include_str!("queries/npm.scm").to_owned()
    }

    fn supports(&self, file: &Path) -> bool {
        file.ends_with("package.json")
    }

    fn version(&self, pkg: &str) -> Cmd {
        Cmd::new("npm", ["view", pkg, "version"])
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, npm::Npm, registry::Registry};

    const JSON: &str = include_str!("../tests/manifests/npm.json");

    #[test]
    fn query() {
        let buf = Buffer::new(JSON.to_owned(), Npm.manifest()).expect("should parse");
        let pairs = buf.query_pairs(&Npm.query_deps()).expect("should query");
        assert_eq!(pairs.len(), 3);
    }
}

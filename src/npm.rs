use std::path::Path;

use crate::{
    buffer::{Lang, Pair},
    proc::Cmd,
    registry::Registry,
};

pub struct Npm;

impl Registry for Npm {
    fn cmd_parse(&self, out: &str) -> Result<String, String> {
        Ok(out.trim_end().to_owned())
    }

    fn cmd_version(&self, pkg: &str) -> Cmd {
        Cmd::new("npm", ["view", pkg, "version"])
    }

    fn dep_parse<'a>(&self, pair: Pair<'a>) -> Result<Pair<'a>, String> {
        Ok(pair)
    }

    fn deps_query(&self) -> String {
        include_str!("queries/npm.scm").to_owned()
    }

    fn manifest(&self) -> Lang {
        Lang::Json
    }

    fn supports(&self, file: &Path) -> bool {
        file.ends_with("package.json")
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, npm::Npm, registry::Registry};

    const JSON: &str = include_str!("../tests/manifests/npm.json");

    #[test]
    fn query() {
        let buf = Buffer::new(JSON.to_owned(), Npm.manifest()).expect("should parse");
        let pairs = buf.query_pairs(&Npm.deps_query()).expect("should query");
        assert_eq!(pairs.len(), 3);
    }
}

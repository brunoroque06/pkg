use std::path::Path;

use crate::{
    buffer::{Buffer, Lang, Pair},
    proc::Cmd,
    registry::Registry,
};

pub struct Pip;

impl Registry for Pip {
    fn cmd_parse(&self, out: &str) -> Result<String, String> {
        let buf = Buffer::new(out.to_owned(), Lang::Json)?;
        let pairs = buf.query_pairs(include_str!("queries/pip_cmd.scm"))?;
        let pair = pairs.first().ok_or("pip output missing lastest version")?;
        Ok(pair.value.to_owned())
    }

    fn cmd_version(&self, pkg: &str) -> Cmd {
        Cmd::new("pip3", ["index", "versions", "--json", pkg])
    }

    fn dep_parse<'a>(&self, pair: Pair<'a>) -> Result<Pair<'a>, String> {
        let sep = "==";
        let val = &pair.value[1..pair.value.len() - 1];
        let (pkg, ver) = val
            .split_once(sep)
            .ok_or(format!("expected name==version dependency {}", val))?;
        let start = pair.range.start + 1 + pkg.len() + sep.len();
        let end = start + ver.len();
        Ok(Pair {
            key: pkg,
            value: ver,
            range: start..end,
        })
    }

    fn deps_query(&self) -> String {
        include_str!("queries/pip.scm").to_owned()
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

    #[test]
    fn query() {
        let buf = Buffer::new(TOML.to_owned(), Pip.manifest()).expect("should parse");
        let pairs = buf.query_pairs(&Pip.deps_query()).expect("should query");
        assert_eq!(pairs.len(), 3);
    }

    #[test]
    fn parse_dep_splits() {
        let buf = Buffer::new(TOML.to_owned(), Pip.manifest()).expect("should parse");
        let pairs = buf.query_pairs(&Pip.deps_query()).expect("should query");
        let pair = pairs.into_iter().next().expect("should have 3");
        let dep = Pip.dep_parse(pair).expect("should parse");
        assert_eq!(dep.key, "lib0");
        assert_eq!(dep.value, "1.2.3");
        assert_eq!(dep.range.start, 68);
        assert_eq!(dep.range.end, 73);
    }
}

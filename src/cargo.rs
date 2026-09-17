use std::path::Path;

use crate::{
    buffer::{Lang, Pair},
    proc::Cmd,
    registry::Registry,
};

pub struct Cargo;

impl Registry for Cargo {
    fn cmd_parse(&self, out: &str) -> Result<String, String> {
        let err = "cannot parse version";
        let version = out
            .lines()
            .find_map(|l| l.trim().strip_prefix("version: "))
            .ok_or(err)?;
        if let Some(s) = version.split_once("(latest ") {
            s.1.strip_suffix(")")
                .map(|v| v.to_owned())
                .ok_or(err.to_owned())
        } else {
            Ok(version.to_owned())
        }
    }

    fn cmd_version(&self, pkg: &str) -> Cmd {
        Cmd::new("cargo", ["info", pkg])
    }

    fn dep_parse<'a>(&self, pair: Pair<'a>) -> Result<Pair<'a>, String> {
        Ok(Pair {
            key: pair.key,
            value: &pair.value[1..pair.value.len() - 1],
            range: (pair.range.start + 1)..(pair.range.end - 1),
        })
    }

    fn deps_query(&self) -> String {
        include_str!("queries/cargo.scm").to_owned()
    }

    fn manifest(&self) -> Lang {
        Lang::Toml
    }

    fn supports(&self, file: &Path) -> bool {
        file.ends_with("Cargo.toml")
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, cargo::Cargo, registry::Registry};

    const CARGO_INFO: &str = include_str!("../tests/cmds/cargo-info.txt");
    const CARGO_INFO_LATEST: &str = include_str!("../tests/cmds/cargo-info-latest.txt");

    const TOML: &str = include_str!("../tests/manifests/cargo.toml");

    #[test]
    fn query() {
        let buf = Buffer::new(TOML.to_owned(), Cargo.manifest()).expect("should parse");
        let pairs = buf.query_pairs(&Cargo.deps_query()).expect("should query");
        assert_eq!(pairs.len(), 3);
    }

    #[test]
    fn cmd_parse() {
        assert_eq!(Cargo.cmd_parse(CARGO_INFO), Ok("0.26.9".to_owned()));
    }

    #[test]
    fn cmd_parse_latest() {
        assert_eq!(Cargo.cmd_parse(CARGO_INFO_LATEST), Ok("0.27.0".to_owned()));
    }

    #[test]
    fn deps_parse_shifts_range() {
        let buf = Buffer::new(TOML.to_owned(), Cargo.manifest()).expect("should parse");
        let pairs = buf.query_pairs(&Cargo.deps_query()).expect("should query");
        let deps = pairs
            .into_iter()
            .map(|p| Cargo.dep_parse(p).expect("should parse"))
            .collect::<Vec<_>>();
        let dep = deps.first().expect("should have");
        assert_eq!(dep.key, "chrono");
        assert_eq!(dep.value, "0.4.45");
        assert_eq!(dep.range.start, 37);
        assert_eq!(dep.range.end, 43);
    }
}

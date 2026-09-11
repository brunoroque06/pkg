use std::path::Path;

use crate::{buffer::Lang, proc::Cmd, registry::Registry};

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
    fn version_parse() {
        assert_eq!(Cargo.cmd_parse(CARGO_INFO), Ok("0.26.9".to_owned()));
    }

    #[test]
    fn version_latest_parse() {
        assert_eq!(
            Cargo.cmd_parse(CARGO_INFO_LATEST),
            Ok("0.27.0".to_owned())
        );
    }
}

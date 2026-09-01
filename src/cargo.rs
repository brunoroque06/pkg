use std::path::Path;

use crate::{buffer::Lang, proc::Cmd, registry::Registry};

pub struct Cargo;

impl Registry for Cargo {
    fn manifest(&self) -> Lang {
        Lang::Toml
    }

    fn parse_version(&self, out: &str) -> Result<String, String> {
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

    fn query_deps(&self) -> String {
        r#"
            (
              (table
                (_) @table
                (pair
                  (_) @key
                  (string) @value))
              (#any-of? @table
                "dependencies"
                "dev-dependencies"
                "build-dependencies")
            )
        "#
        .to_owned()
    }

    fn supports(&self, file: &Path) -> bool {
        file.ends_with("Cargo.toml")
    }

    fn version(&self, pkg: &str) -> Cmd {
        Cmd::new("cargo", ["info", pkg])
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, cargo::Cargo, registry::Registry};

    const CARGO_INFO: &str = r#"
        tree-sitter #incremental #parsing
        Rust bindings to the Tree-sitter parsing library
        version: 0.26.9
        license: MIT
        rust-version: 1.77
    "#;

    const CARGO_INFO_LATEST: &str = r#"
        tree-sitter #incremental #parsing
        Rust bindings to the Tree-sitter parsing library
        version: 0.26.9 (latest 0.27.0)
        license: MIT
        rust-version: 1.77
    "#;

    const TOML: &str = r#"
        [dependencies]
        tree-sitter = "0.26.9"
        tree-sitter-json = "0.24.8"
    "#;

    #[test]
    fn query() {
        let buf = Buffer::new(TOML.to_owned(), Cargo.manifest()).expect("should parse");
        let pairs = buf.query_pairs(&Cargo.query_deps()).expect("should query");
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn version_parse() {
        assert_eq!(Cargo.parse_version(CARGO_INFO), Ok("0.26.9".to_owned()));
    }

    #[test]
    fn version_latest_parse() {
        assert_eq!(
            Cargo.parse_version(CARGO_INFO_LATEST),
            Ok("0.27.0".to_owned())
        );
    }
}

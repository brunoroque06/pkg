use std::path::Path;

use crate::{buffer::Lang, proc::Cmd, registry::Registry};

pub struct Npm;

impl Registry for Npm {
    fn supports(&self, file: &Path) -> bool {
        file.ends_with("package.json")
    }

    fn query_deps(&self) -> String {
        r#"
        (
          (document
            (object
              (pair
                key: (string
                  (string_content) @group)
                value: (object
                  (pair
                    key: (string
                      (string_content) @key)
                    value: (string
                      (string_content) @value))))))
          (#any-of? @group
            "dependencies"
            "devDependencies")
        )
        "#
        .to_owned()
    }

    fn version(&self, pkg: &str) -> Cmd {
        Cmd::new("npm", ["view", pkg, "version"])
    }

    fn parse_version(&self, out: &str) -> Result<String, String> {
        Ok(out.trim_end().to_owned())
    }

    fn manifest(&self) -> Lang {
        Lang::Json
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, npm::Npm, registry::Registry};

    const JSON: &str = r#"{"name": "app", "ver": "0.0.1", "dependencies": {"lib": "22.3", "lib2": "9.7"}, "devDependencies": {"lib3": "28.10"}}"#;

    #[test]
    fn query() {
        let buf = Buffer::new(JSON.to_owned(), Npm.manifest()).expect("should parse");
        let pairs = buf.query_pairs(&Npm.query_deps()).expect("should query");
        assert_eq!(pairs.len(), 3);
    }
}

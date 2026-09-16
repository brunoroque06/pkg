use std::{cmp::Reverse, ops::Range};

use tree_sitter::{Language, Node, Parser, Query, QueryCursor, StreamingIteratorMut};

pub enum Lang {
    Json,
    Toml,
}

impl Lang {
    fn language(&self) -> Language {
        match self {
            Lang::Json => tree_sitter_json::LANGUAGE.into(),
            Lang::Toml => tree_sitter_toml_ng::LANGUAGE.into(),
        }
    }
}

pub struct Buffer {
    src: String,
    tree: tree_sitter::Tree,
}

pub struct Pair<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub range: Range<usize>,
}

pub struct Edit<'a> {
    pair: Pair<'a>,
    value: String,
}

impl<'a> Pair<'a> {
    pub fn with_value(self, value: String) -> Edit<'a> {
        Edit { pair: self, value }
    }
}

impl Buffer {
    pub fn new(src: String, lang: Lang) -> Result<Buffer, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&lang.language())
            .map_err(|e| e.to_string())?;
        let fail = "failed to parse source";
        let tree = parser.parse(&src, None).ok_or_else(|| fail.to_owned())?;
        if tree.root_node().has_error() {
            return Err(fail.to_owned());
        }
        Ok(Buffer { src, tree })
    }

    pub fn query_pairs(&self, query: &str) -> Result<Vec<Pair<'_>>, String> {
        let query = Query::new(&self.tree.language(), query).map_err(|e| e.to_string())?;

        let index = |id: &str| {
            query
                .capture_index_for_name(id)
                .ok_or(format!("query must capture @{}", id))
        };

        let key_id = index("key")?;
        let value_id = index("value")?;

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, self.tree.root_node(), self.src.as_bytes());

        let mut pairs = Vec::new();

        while let Some(m) = matches.next_mut() {
            let get_id = |id: &str, idx: u32| {
                m.captures()
                    .iter()
                    .find(|c| c.index == idx)
                    .map(|c| c.node)
                    .ok_or(format!("match missing @{}", id))
            };
            let key_node = get_id("key", key_id)?;
            let value_node = get_id("value", value_id)?;

            let get_value =
                |n: Node<'_>| n.utf8_text(self.src.as_bytes()).map_err(|e| e.to_string());
            let key = get_value(key_node)?;
            let value = get_value(value_node)?;

            pairs.push(Pair {
                key,
                value,
                range: value_node.byte_range(),
            });
        }

        Ok(pairs)
    }

    pub fn replace(&self, mut edits: Vec<Edit>) -> String {
        edits.sort_unstable_by_key(|e| Reverse(e.pair.range.end));

        let mut src = self.src.clone();

        for e in edits {
            src.replace_range(e.pair.range, &e.value);
        }

        src
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &str = include_str!("../tests/manifests/npm.json");
    const JSON_EMPTY: &str = include_str!("../tests/manifests/empty.json");
    const JSON_INVALID: &str = include_str!("../tests/manifests/invalid.json");
    const JSON_QUERY: &str = include_str!("queries/npm.scm");
    const JSON_QUERY_INVALID: &str = include_str!("../tests/queries/invalid.scm");

    const TOML: &str = include_str!("../tests/manifests/cargo.toml");
    const TOML_EMPTY: &str = include_str!("../tests/manifests/empty.toml");
    const TOML_INVALID: &str = include_str!("../tests/manifests/invalid.toml");

    #[test]
    fn new_invalid_source() {
        assert!(Buffer::new(JSON_INVALID.to_owned(), Lang::Json).is_err());
        assert!(Buffer::new(TOML_INVALID.to_owned(), Lang::Toml).is_err());
    }

    #[test]
    fn new_wrong_lang() {
        assert!(Buffer::new(JSON.to_owned(), Lang::Toml).is_err());
        assert!(Buffer::new(TOML.to_owned(), Lang::Json).is_err());
    }

    #[test]
    fn new_valid() {
        assert!(Buffer::new(JSON_EMPTY.to_owned(), Lang::Json).is_ok());
        assert!(Buffer::new(JSON.to_owned(), Lang::Json).is_ok());
        assert!(Buffer::new(TOML_EMPTY.to_owned(), Lang::Toml).is_ok());
        assert!(Buffer::new(TOML.to_owned(), Lang::Toml).is_ok());
    }

    fn parse(src: &str, lang: Lang) -> Buffer {
        Buffer::new(src.to_owned(), lang).expect("should parse")
    }

    #[test]
    fn query_pairs_invalid_query() {
        assert!(parse(JSON, Lang::Json).query_pairs("()").is_err());
    }

    #[test]
    fn query_pairs_missing_matches() {
        assert!(
            parse(JSON, Lang::Json)
                .query_pairs(JSON_QUERY_INVALID)
                .is_err()
        );
    }

    impl Buffer {
        pub fn query_json(&self) -> Vec<Pair<'_>> {
            self.query_pairs(JSON_QUERY).expect("should query")
        }
    }

    #[test]
    fn query_pairs() {
        let buf = parse(JSON, Lang::Json);
        let pairs = buf.query_json();
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0].key, "lib0");
        assert_eq!(pairs[0].value, "22.3");
    }

    #[test]
    fn replace_json() {
        let buf_o = parse(JSON, Lang::Json);
        let leaves_o = buf_o.query_json();
        let edits = leaves_o
            .into_iter()
            .map(|l| l.with_value("dummy".to_owned()))
            .collect();
        let buf = parse(&buf_o.replace(edits), Lang::Json);
        let leaves = buf.query_json();

        assert_eq!(leaves.len(), 3);
        assert_eq!(leaves[0].key, "lib0");
        assert_eq!(leaves[0].value, "dummy");
        assert_eq!(leaves[1].key, "lib1");
        assert_eq!(leaves[1].value, "dummy");
    }
}

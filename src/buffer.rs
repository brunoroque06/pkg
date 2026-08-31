use tree_sitter::{Language, Node, Parser};

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
    pub(crate) key: &'a str,
    pub(crate) value: &'a str,
    node: Node<'a>,
}

impl Buffer {
    pub fn new(src: String, typ: Lang) -> Result<Buffer, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&typ.language())
            .map_err(|e| e.to_string())?;
        let fail = "failed to parse source";
        let tree = parser.parse(&src, None).ok_or_else(|| fail.to_owned())?;
        if tree.root_node().has_error() {
            return Err(fail.to_owned());
        }
        Ok(Buffer { src, tree })
    }

    pub fn get_pairs(&self, key: &str) -> Option<Vec<Pair<'_>>> {
        let root = self.tree.root_node().named_child(0)?;
        let Some(parent) = get_node(root, &self.src, key) else {
            return Some(Vec::new());
        };
        let mut cursor = parent.walk();

        let leaves = parent
            .named_children(&mut cursor)
            .map(|child| {
                let key_node = child.child_by_field_name("key")?;
                let value_node = child.child_by_field_name("value")?;

                Some(Pair {
                    key: key_node.utf8_text(self.src.as_bytes()).ok()?,
                    value: value_node.utf8_text(self.src.as_bytes()).ok()?,
                    node: value_node,
                })
            })
            .collect::<Option<Vec<_>>>()?;

        Some(leaves)
    }
}

fn clean(text: &str) -> &str {
    text.strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .or_else(|| text.strip_prefix('\'').and_then(|t| t.strip_suffix('\'')))
        .unwrap_or(text)
}

fn get_node<'a>(node: Node<'a>, src: &str, key: &str) -> Option<Node<'a>> {
    node.named_children(&mut node.walk())
        .find(|p| {
            p.child_by_field_name("key")
                .and_then(|k| k.utf8_text(src.as_bytes()).ok())
                == Some(&format!("\"{key}\""))
        })?
        .child_by_field_name("value")
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON_INVALID: &str = "{";
    const JSON_EMPTY: &str = "{}";
    const JSON: &str = r#"{"name": "app", "ver": "0.0.1", "deps": {"lib": "22.3", "lib2": "9.7"}, "devDeps": {"fmt": "28.10"}}"#;

    #[test]
    fn new_invalid() {
        assert!(Buffer::new(JSON_INVALID.to_owned(), Lang::Json).is_err());
    }

    #[test]
    fn new_valid() {
        assert!(Buffer::new(JSON_EMPTY.to_owned(), Lang::Json).is_ok());
        assert!(Buffer::new(JSON.to_owned(), Lang::Json).is_ok());
    }

    #[test]
    fn children() {
        let syntax = Buffer::new(JSON.to_owned(), Lang::Json).expect("should parse");
        let leaves = syntax.get_pairs("deps").expect("should find");
        assert_eq!(leaves.len(), 2);
        assert_eq!(leaves[0].key, "\"lib\"");
        assert_eq!(leaves[0].value, "\"22.3\"");
        assert_eq!(leaves[1].key, "\"lib2\"");
        assert_eq!(leaves[1].value, "\"9.7\"");
    }
}

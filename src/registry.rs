use std::path::Path;

use crate::{buffer::Lang, proc::Cmd};

pub trait Registry {
    fn cmd_parse(&self, cmd: &str) -> Result<String, String>;
    fn cmd_version(&self, pkg: &str) -> Cmd;
    fn deps_query(&self) -> String;
    fn manifest(&self) -> Lang;
    fn supports(&self, file: &Path) -> bool;
}

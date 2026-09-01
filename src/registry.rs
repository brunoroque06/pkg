use std::path::Path;

use crate::{buffer::Lang, proc::Cmd};

pub trait Registry {
    fn manifest(&self) -> Lang;
    fn parse_version(&self, cmd: &str) -> Result<String, String>;
    fn query_deps(&self) -> String;
    fn supports(&self, file: &Path) -> bool;
    fn version(&self, pkg: &str) -> Cmd;
}

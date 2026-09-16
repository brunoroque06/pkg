use std::path::Path;

use crate::{
    buffer::{Lang, Pair},
    proc::Cmd,
};

pub trait Registry {
    fn cmd_parse(&self, cmd: &str) -> Result<String, String>;
    fn cmd_version(&self, pkg: &str) -> Cmd;
    fn dep_parse<'a>(&self, pair: Pair<'a>) -> Result<Pair<'a>, String>;
    fn deps_query(&self) -> String;
    fn manifest(&self) -> Lang;
    fn supports(&self, file: &Path) -> bool;
}

use std::path::Path;

use crate::{buffer::Lang, proc::Cmd};

pub trait Registry {
    fn can_handle(&self, file: &Path) -> bool;
    fn latest_version(&self, pkg: &str) -> Cmd;
    fn latest_version_parse(&self, out: &str) -> String;
    fn manifest_type(&self) -> Lang;
}

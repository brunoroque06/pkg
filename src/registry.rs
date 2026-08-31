use crate::{buffer::Lang, proc::Cmd};

pub trait Registry {
    fn can_handle(file: &str) -> bool;
    fn latest_version(pkg: &str) -> Cmd;
    fn latest_version_parse(out: &str) -> String;
    fn manifest_type() -> Lang;
}

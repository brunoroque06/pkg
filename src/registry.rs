use crate::buffer::Lang;

pub trait Registry {
    fn can_handle(file: &str) -> bool;
    fn cmd_latest_version(pkg: &str) -> String;
    fn manifest_type() -> Lang;
}

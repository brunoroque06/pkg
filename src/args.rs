use argh::FromArgs;

use crate::buffer::Position;

/// pkg
#[derive(Debug, FromArgs)]
pub struct Args {
    /// concurrency
    #[argh(option, default = "4", short = 'c')]
    pub concurrency: usize,

    #[argh(positional)]
    pub file: String,

    /// position to filter: LINE or LINE:COL (1-based)
    #[argh(option, short = 'p')]
    pub position: Option<Position>,
}

pub fn parse_args() -> Args {
    argh::from_env()
}

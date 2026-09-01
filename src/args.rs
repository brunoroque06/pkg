use argh::FromArgs;

/// pkg
#[derive(Debug, FromArgs)]
pub struct Args {
    /// concurrency
    #[argh(option, default = "4")]
    pub concurrency: usize,

    #[argh(positional)]
    pub file: String,
}

pub fn parse_args() -> Args {
    argh::from_env()
}

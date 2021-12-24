use clap::{ColorChoice, Parser};

#[derive(Parser, Debug)]
#[clap(about, version, name = "tcracker", color(ColorChoice::Never))]
pub struct Args {
    /// The APP.app path
    #[clap(required = true)]
    pub(crate) app_path: String,
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
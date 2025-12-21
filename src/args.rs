use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub day: Option<u8>
}

impl Args {
    pub fn build() -> Self {
        Self::parse()
    }
}

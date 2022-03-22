use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the challenge to run
    #[clap(short, long)]
    name: String
}
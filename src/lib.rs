pub use clap::Parser;
pub mod bitvec_set;
pub mod grid_util;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input: String,
}

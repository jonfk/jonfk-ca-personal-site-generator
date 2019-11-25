use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jonfk_ca")]
pub struct Opt {
    /// Delete target_dir on build
    #[structopt(short, long)]
    pub disable_rm_target: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// input contents directory
    #[structopt(short, long, parse(from_os_str), default_value = "content")]
    pub input: PathBuf,

    /// Output directory
    #[structopt(short, long, parse(from_os_str), default_value = "public")]
    pub output: PathBuf,
}

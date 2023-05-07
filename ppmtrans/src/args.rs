// clap parser for command line arguments

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]

pub struct Args {

    // transpose 
    #[clap(long = "transpose", required = false)]
    pub transpose: bool,

    /// How to flip it?
    #[clap(long = "flip", required = false)]
    pub flip: Option<String>,

    /// degree of rotation
    #[clap(short = 'r', long = "rotate")]
    pub rotate: Option<u32>,

    /// row major file to iterate
    #[clap(long = "row-major", required = false)]
    pub r_fname: bool,
            
    /// col major file to iterate
    #[clap(long = "col-major", required = false)]
    pub c_fname: bool,

    #[clap()]
    pub fname: Option<String>,
}

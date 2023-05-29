use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // X dimension of the board
    #[arg(short, long)]
    pub x: u32,
    // Y dimension of the board
    #[arg(short, long)]
    pub y: u32,

    // Number of initial alive cells
    #[arg(short, long)]
    pub p: u32,

    // Number of iterations to perform
    #[arg(short, long)]
    pub i: u32,

    // Verbosity
    #[arg(short, long)]
    pub v: bool,
}

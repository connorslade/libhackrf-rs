use anyhow::Result;
use args::{Args, Command};
use clap::Parser;

mod args;
mod commands;
mod consts;
mod filters;
mod signal;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Transmit(args) => commands::transmit::run(args),
        Command::Receive(args) => commands::receive::run(args),
    }
}

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Transmit a mono audio file over FM.
    Transmit(TransmitArgs),
    /// Receive a mono FM transmission.
    Receive(ReceiveArgs),
}

#[derive(Parser)]
pub struct TransmitArgs {
    /// The center frequency to transmit on.
    #[arg(short, long, default_value_t = 100_000_000)]
    pub frequency: u64,
    /// The transmit variable gain amplifier power setting. (In db)
    #[arg(short, long, default_value_t = 30)]
    pub gain: u32,

    /// Path to a .wav file to transmit. Only the first channel is proessed.
    pub audio: PathBuf,
}

#[derive(Parser)]
pub struct ReceiveArgs {
    /// The center frequency to receive
    #[arg(short, long, default_value_t = 100_000_000)]
    pub frequency: u64,
    /// The receive variable gain amplifier power setting. (In db)
    #[arg(short, long, default_value_t = 0)]
    pub gain: u32,
    /// The receive low noise amplifier power setting. (In db)
    #[arg(short, long, default_value_t = 30)]
    pub lna_gain: u32,

    /// Path of a .wav file that will be created and written to.
    pub audio: PathBuf,
}

use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use consts::{SAMPLE_RATE, TX_BANDWIDTH};
use hound::WavReader;

mod consts;
mod filters;
pub mod hackrf;
mod signal;
use hackrf::{util::ToComplexI8, HackRf};
use signal::modulate::Modulator;

fn main() -> Result<()> {
    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(SAMPLE_RATE)?;
    hackrf.set_freq(100_000_000)?;

    hackrf.set_amp_enable(true)?;
    hackrf.set_transmit_gain(20)?;
    hackrf.set_baseband_filter_bandwidth(SAMPLE_RATE)?;

    let serial_number = hackrf.get_serial_number()?;
    println!(
        "Connected to: {}\n",
        serial_number
            .serial_no
            .iter()
            .map(|x| format!("{:08X}", x))
            .collect::<Vec<_>>()
            .join("-")
    );

    let wav = WavReader::open("/home/connorslade/Downloads/taxi-f32.wav").unwrap();
    let audio = Arc::new(Mutex::new(Modulator::new(SAMPLE_RATE, TX_BANDWIDTH, wav)));
    hackrf.start_tx(
        |_hackrf, buffer, user| {
            let data = user.downcast_ref::<Arc<Mutex<Modulator>>>().unwrap();
            let mut data = data.lock().unwrap();

            buffer.iter_mut().for_each(|x| *x = data.sample().to_i8());
        },
        audio.clone(),
    )?;

    loop {
        let progress = audio.lock().unwrap().progress();
        print!("\rTransmitting: {:.2}%", progress * 100.0);
        stdout().flush().unwrap();

        if progress == 1.0 {
            break;
        }
    }

    hackrf.stop_tx()?;

    Ok(())
}

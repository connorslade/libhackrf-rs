use std::{
    io::stdin,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use hound::WavWriter;
use libhackrf::{util::ToComplexF32, HackRf};

use crate::{
    args::ReceiveArgs,
    consts::{SAMPLE_RATE, WAVE_SPEC},
    signal::demodulate::Demodulator,
};

pub fn run(args: ReceiveArgs) -> Result<()> {
    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(SAMPLE_RATE)?;
    hackrf.set_freq(args.frequency)?;
    hackrf.set_lna_gain(args.lna_gain)?;
    hackrf.set_rxvga_gain(args.gain)?;

    let serial_number = hackrf.get_serial_number()?;
    println!(
        "Connected to: {}",
        serial_number
            .serial_no
            .iter()
            .map(|x| format!("{:08X}", x))
            .collect::<Vec<_>>()
            .join("-")
    );

    // Really could be an UnsafeCell, but whatecer
    let audio = Arc::new(Mutex::new((Demodulator::new(), Vec::<f32>::new())));
    hackrf.start_rx(
        |_hackrf, buffer, user| {
            let data = user
                .downcast_ref::<Arc<Mutex<(Demodulator, Vec<f32>)>>>()
                .unwrap();
            let mut data = data.lock().unwrap();

            let samples = buffer.iter().map(|x| x.to_f32()).collect::<Vec<_>>();

            data.0.replace(samples);
            let audio = data.0.audio(-900e3, 1.0);
            data.1.extend_from_slice(&audio);
        },
        audio.clone(),
    )?;

    println!("Press Enter to stop recording...");

    let mut string = String::new();
    stdin().read_line(&mut string)?;
    hackrf.stop_tx()?;

    let mut writer = WavWriter::create(args.audio, WAVE_SPEC)?;
    for sample in audio.lock().unwrap().1.iter() {
        writer.write_sample(*sample)?;
    }
    writer.finalize()?;

    Ok(())
}

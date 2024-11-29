use std::{cell::RefCell, io::stdin, rc::Rc};

use anyhow::Result;
use consts::{SAMPLE_RATE, WAVE_SPEC};
use hound::WavWriter;
use num_complex::Complex;

mod consts;
mod filters;
mod hackrf;
mod signal;
use hackrf::HackRf;
use signal::demodulate::Demodulator;

fn main() -> Result<()> {
    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(SAMPLE_RATE)?;
    hackrf.set_freq(100_000_000)?;

    hackrf.set_amp_enable(true)?;
    hackrf.set_lna_gain(32)?;
    hackrf.set_rxvga_gain(0)?;

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

    let audio = Rc::new(RefCell::new((Demodulator::new(), Vec::<f32>::new())));
    hackrf.start_rx(
        |_hackrf, buffer, user| {
            let data = user
                .downcast_ref::<Rc<RefCell<(Demodulator, Vec<f32>)>>>()
                .unwrap();
            let mut data = data.borrow_mut();

            let samples = buffer
                .iter()
                .map(|x| Complex::new(x.re as f32 / 127.0, x.im as f32 / 127.0))
                .collect::<Vec<_>>();

            data.0.replace(samples);
            let audio = data.0.audio(-900e3, 1.0);
            data.1.extend_from_slice(&audio);
        },
        audio.clone(),
    )?;

    let mut string = String::new();
    stdin().read_line(&mut string)?;
    hackrf.stop_tx()?;

    let mut writer = WavWriter::create("output.wav", WAVE_SPEC)?;
    for sample in audio.borrow().1.iter() {
        writer.write_sample(*sample)?;
    }
    writer.finalize()?;

    Ok(())
}

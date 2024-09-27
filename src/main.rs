use std::{io::stdin, sync::atomic::AtomicU64};

use anyhow::Result;

use hackrf::HackRf;
mod hackrf;

fn main() -> Result<()> {
    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(8_000_000)?;
    hackrf.set_freq(100_000_000)?;
    hackrf.set_transmit_gain(10)?;

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

    hackrf.start_tx(
        |_hackrf, buffer, user| {
            let now = user.downcast_ref::<AtomicU64>().unwrap();
            let n = now.load(std::sync::atomic::Ordering::Relaxed);

            let sample_rate = 8_000_000;
            let center_freq = 1_000_000;

            let mut next = 0;
            for (idx, iq) in buffer.chunks_mut(2).enumerate() {
                let t = (n + idx as u64) as f64 / sample_rate as f64;
                let carrier_signal = (2.0 * std::f64::consts::PI * center_freq as f64 * t).sin();

                iq[0] = unsafe { std::mem::transmute((carrier_signal * 127.0 + 127.0) as i8) };
                iq[1] = 0;

                next += 1;
            }

            now.store(next, std::sync::atomic::Ordering::Relaxed);
        },
        AtomicU64::new(0),
    )?;

    let mut string = String::new();
    stdin().read_line(&mut string)?;

    hackrf.stop_tx()?;
    Ok(())
}

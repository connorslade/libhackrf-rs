use std::io::stdin;

use anyhow::Result;

use hackrf::HackRf;
mod hackrf;

fn main() -> Result<()> {
    let hackrf = HackRf::open()?;
    hackrf.set_sample_rate(8_000_000)?;
    hackrf.set_freq(100_000)?;

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

    hackrf.start_tx(|_hackrf, buffer| {
        println!("Buffer: {:?}", buffer.len());
    })?;

    let mut string = String::new();
    stdin().read_line(&mut string)?;

    hackrf.stop_tx()?;
    Ok(())
}

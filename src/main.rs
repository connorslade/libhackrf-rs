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

    let mut n = 0_usize;
    hackrf.start_tx(
        |_hackrf, _buffer, user| unsafe {
            let user = user as *mut usize;
            *user += 1;
            println!("Callback: {}", *user);
        },
        &mut n as *mut _ as *mut std::ffi::c_void,
    )?;

    let mut string = String::new();
    stdin().read_line(&mut string)?;

    hackrf.stop_tx()?;
    Ok(())
}

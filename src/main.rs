use anyhow::Result;

use hackrf::HackRf;
mod hackrf;

fn main() -> Result<()> {
    let hackrf = HackRf::open()?;
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

    Ok(())
}

use std::ptr;

use hackrf::{SerialNumber, HACKRF_SUCCESS};

mod hackrf;

fn main() {
    unsafe {
        assert_eq!(hackrf::hackrf_init(), HACKRF_SUCCESS);

        let mut device = ptr::null_mut();
        assert_eq!(hackrf::hackrf_open(&mut device), HACKRF_SUCCESS);

        let mut serial_number = SerialNumber::default();
        assert_eq!(
            hackrf::hackrf_board_partid_serialno_read(device, &mut serial_number),
            HACKRF_SUCCESS
        );

        println!(
            "Part ID: {:08X}-{:08X}",
            serial_number.part_id[0], serial_number.part_id[1]
        );
    }
}

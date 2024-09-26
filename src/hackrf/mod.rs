use std::sync::atomic::{AtomicUsize, Ordering};

pub mod error;
mod ffi;

use error::{HackrfError, Result};
use ffi::SerialNumber;

static DEVICE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct HackRf {
    device: *mut ffi::HackrfDevice,
}

impl HackRf {
    pub fn open() -> Result<Self> {
        if DEVICE_COUNT.fetch_add(1, Ordering::Relaxed) == 0 {
            unsafe { HackrfError::from_id(ffi::hackrf_init())? }
        }

        let mut device = std::ptr::null_mut();
        unsafe { HackrfError::from_id(ffi::hackrf_open(&mut device))? }

        Ok(Self { device })
    }

    pub fn get_serial_number(&self) -> Result<SerialNumber> {
        let mut serial_number = SerialNumber::default();
        unsafe {
            HackrfError::from_id(ffi::hackrf_board_partid_serialno_read(
                self.device,
                &mut serial_number,
            ))?
        }
        Ok(serial_number)
    }
}

impl Drop for HackRf {
    fn drop(&mut self) {
        let _ = unsafe { HackrfError::from_id(ffi::hackrf_close(self.device)) };

        if DEVICE_COUNT.fetch_sub(1, Ordering::Relaxed) == 1 {
            let _ = unsafe { HackrfError::from_id(ffi::hackrf_exit()) };
        }
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};

pub mod error;
pub mod ffi;

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

    pub fn set_freq(&self, freq: u64) -> Result<()> {
        unsafe { HackrfError::from_id(ffi::hackrf_set_freq(self.device, freq)) }
    }

    pub fn set_sample_rate(&self, sample_rate: u32) -> Result<()> {
        unsafe {
            HackrfError::from_id(ffi::hackrf_set_sample_rate_manual(
                self.device,
                sample_rate,
                1,
            ))
        }
    }

    pub fn start_tx(&self, callback: extern "C" fn(*mut ffi::HackrfTransfer) -> i32) -> Result<()> {
        unsafe {
            HackrfError::from_id(ffi::hackrf_start_tx(
                self.device,
                callback,
                std::ptr::null_mut(),
            ))
        }
    }

    pub fn stop_tx(&self) -> Result<()> {
        unsafe { HackrfError::from_id(ffi::hackrf_stop_tx(self.device)) }
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

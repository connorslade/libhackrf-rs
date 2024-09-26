use std::{
    any::Any,
    ffi::c_void,
    ops::Deref,
    ptr,
    sync::{
        atomic::{AtomicPtr, AtomicUsize, Ordering},
        Arc,
    },
};

pub mod error;
mod ffi;
mod transfer;

use error::{HackrfError, Result};
use ffi::SerialNumber;
use transfer::{tx_callback, TransferCallback, TransferContext};

static DEVICE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct HackRf {
    inner: Arc<HackRfInner>,
}

pub struct HackRfInner {
    device: *mut ffi::HackrfDevice,
    ctx_tx: AtomicPtr<c_void>,
}

impl HackRf {
    pub fn open() -> Result<HackRf> {
        if DEVICE_COUNT.fetch_add(1, Ordering::Relaxed) == 0 {
            unsafe { HackrfError::from_id(ffi::hackrf_init())? }
        }

        let mut device = std::ptr::null_mut();
        unsafe { HackrfError::from_id(ffi::hackrf_open(&mut device))? }

        Ok(Self {
            inner: Arc::new(HackRfInner {
                device,
                ctx_tx: AtomicPtr::new(ptr::null_mut()),
            }),
        })
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

    pub fn start_tx(&self, callback: TransferCallback, user_data: impl Any) -> Result<()> {
        let context = TransferContext::new(callback, self.clone(), Box::new(user_data));
        let callback = Box::leak(Box::new(context)) as *mut _ as *mut _;
        self.ctx_tx.store(callback, Ordering::Relaxed);

        unsafe { HackrfError::from_id(ffi::hackrf_start_tx(self.device, tx_callback, callback)) }
    }

    pub fn stop_tx(&self) -> Result<()> {
        let callback = self.ctx_tx.swap(ptr::null_mut(), Ordering::Relaxed);
        if !callback.is_null() {
            let callback = unsafe { Box::from_raw(callback as *mut fn(*mut ffi::HackrfTransfer)) };
            drop(callback);
        }

        unsafe { HackrfError::from_id(ffi::hackrf_stop_tx(self.device)) }
    }
}

unsafe impl Send for HackRfInner {}
unsafe impl Sync for HackRfInner {}

impl Deref for HackRf {
    type Target = HackRfInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
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

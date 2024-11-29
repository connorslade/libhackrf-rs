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
use transfer::{rx_callback, tx_callback, ReceiveCallback, TransferContext, TransmitCallback};

static DEVICE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct HackRf {
    inner: Arc<HackRfInner>,
}

pub struct HackRfInner {
    device: *mut ffi::HackrfDevice,
    user_data: AtomicPtr<c_void>,
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
                user_data: AtomicPtr::new(ptr::null_mut()),
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

    pub fn set_amp_enable(&self, enable: bool) -> Result<()> {
        unsafe { HackrfError::from_id(ffi::hackrf_set_amp_enable(self.device, enable as u8)) }
    }

    /// Between 0db and 47db.
    pub fn set_transmit_gain(&self, gain: u32) -> Result<()> {
        unsafe { HackrfError::from_id(ffi::hackrf_set_txvga_gain(self.device, gain)) }
    }

    /// Low noise amplifier gain.
    /// Between 0d and 40d in steps of 8dB.
    pub fn set_lna_gain(&self, gain: u32) -> Result<()> {
        unsafe { HackrfError::from_id(ffi::hackrf_set_lna_gain(self.device, gain)) }
    }

    pub fn set_rxvga_gain(&self, gain: u32) -> Result<()> {
        unsafe { HackrfError::from_id(ffi::hackrf_set_vga_gain(self.device, gain)) }
    }

    pub fn set_txvga_gain(&self, gain: u32) -> Result<()> {
        unsafe { HackrfError::from_id(ffi::hackrf_set_txvga_gain(self.device, gain)) }
    }

    /// Variable gain amplifier gain.
    /// Between 0db and 62db in steps of 2dB.
    pub fn set_gain(&self, gain: u32) -> Result<()> {
        unsafe { HackrfError::from_id(ffi::hackrf_set_vga_gain(self.device, gain)) }
    }

    pub fn start_tx(&self, callback: TransmitCallback, user_data: impl Any) -> Result<()> {
        let context = TransferContext::new(callback, self.clone(), Box::new(user_data));
        let callback = Box::leak(Box::new(context)) as *mut _ as *mut _;
        self.user_data.store(callback, Ordering::Relaxed);

        unsafe { HackrfError::from_id(ffi::hackrf_start_tx(self.device, tx_callback, callback)) }
    }

    pub fn stop_tx(&self) -> Result<()> {
        let callback = self.user_data.swap(ptr::null_mut(), Ordering::Relaxed);
        if !callback.is_null() {
            let callback = unsafe { Box::from_raw(callback as *mut fn(*mut ffi::HackrfTransfer)) };
            drop(callback);
        }

        unsafe { HackrfError::from_id(ffi::hackrf_stop_tx(self.device)) }
    }

    pub fn start_rx(&self, callback: ReceiveCallback, user_data: impl Any) -> Result<()> {
        let context = TransferContext::new(callback, self.clone(), Box::new(user_data));
        let callback = Box::leak(Box::new(context)) as *mut _ as *mut _;
        self.user_data.store(callback, Ordering::Relaxed);

        unsafe { HackrfError::from_id(ffi::hackrf_start_rx(self.device, rx_callback, callback)) }
    }

    pub fn stop_rx(&self) -> Result<()> {
        let callback = self.user_data.swap(ptr::null_mut(), Ordering::Relaxed);
        if !callback.is_null() {
            let callback = unsafe { Box::from_raw(callback as *mut fn(*mut ffi::HackrfTransfer)) };
            drop(callback);
        }

        unsafe { HackrfError::from_id(ffi::hackrf_stop_rx(self.device)) }
    }

    pub fn is_streaming(&self) -> bool {
        unsafe { ffi::hackrf_is_streaming(self.device) != 0 }
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

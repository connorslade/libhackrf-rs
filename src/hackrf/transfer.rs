use std::slice;

use super::{ffi, HackRf};

pub type TransferCallback = fn(&HackRf, &mut [u8]);

pub struct TransferContext {
    callback: TransferCallback,
    hackrf: HackRf,
}

impl TransferContext {
    pub(super) fn new(callback: TransferCallback, hackrf: HackRf) -> Self {
        Self { callback, hackrf }
    }
}

pub(super) extern "C" fn tx_callback(transfer: *mut ffi::HackrfTransfer) -> i32 {
    unsafe {
        let transfer = &mut *transfer;
        let context = &*(transfer.tx_ctx as *mut TransferContext);

        let buffer = slice::from_raw_parts_mut(transfer.buffer, transfer.buffer_length as usize);
        (context.callback)(&context.hackrf, buffer);
    }

    0
}

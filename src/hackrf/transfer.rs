use std::{any::Any, slice};

use super::{ffi, HackRf};

pub type TransferCallback = fn(&HackRf, &mut [u8], &dyn Any);

pub struct TransferContext {
    callback: TransferCallback,
    hackrf: HackRf,
    user_data: Box<dyn Any>,
}

impl TransferContext {
    pub(super) fn new(callback: TransferCallback, hackrf: HackRf, user_data: Box<dyn Any>) -> Self {
        Self {
            callback,
            hackrf,
            user_data,
        }
    }
}

pub(super) extern "C" fn tx_callback(transfer: *mut ffi::HackrfTransfer) -> i32 {
    unsafe {
        let transfer = &mut *transfer;
        let context = &*(transfer.tx_ctx as *mut TransferContext);

        let buffer = slice::from_raw_parts_mut(transfer.buffer, transfer.buffer_length as usize);
        (context.callback)(&context.hackrf, buffer, &*context.user_data);
    }

    0
}

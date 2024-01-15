use std::{
    alloc::LayoutError,
    cell::BorrowError,
    env::VarError,
    ffi::NulError,
    str::Utf8Error,
    string::{FromUtf8Error, ParseError},
    sync::{
        mpsc::{RecvError, SendError, TryRecvError, TrySendError},
        PoisonError, TryLockError,
    },
};

fn main() {
    // NulError;
    // VarError;
    // RecvError;
    // SendError;
    // Utf8Error;
    // ParseError;
    // BorrowError;
    // LayoutError;
    // PoisonError;
    // TryLockError;
    // TryRecvError;
    // TrySendError;
    // FromUtf8Error;

    // Err(0);
    // Ok(1);
}

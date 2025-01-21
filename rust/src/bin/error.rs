use std::{
    alloc::LayoutError,
    cell::BorrowError,
    env::VarError,
    ffi::NulError,
    io::Error,
    str::Utf8Error,
    string::{FromUtf8Error, ParseError},
    sync::{
        mpsc::{RecvError, SendError, TryRecvError, TrySendError},
        PoisonError, TryLockError,
    },
};

fn read_file(path: &str) -> Result<String, Error> {
    std::fs::read_to_string(path)
}

fn main() {
    let path = "unknow.file";
    match read_file(path) {
        Ok(file) => {
            println!("{}", file)
        }
        Err(e) => {
            println!("{} {}", path, e)
        }
    }
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

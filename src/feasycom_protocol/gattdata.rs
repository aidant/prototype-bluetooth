use alloc::string::{String, ToString};
use core::str::{self, Utf8Error};

pub type GattData = String;

pub fn parse(value: &[u8]) -> Result<GattData, Utf8Error> {
    let (len, data) = value.split_once(|c| c == &b',').unwrap_or((&[], value));

    Ok(str::from_utf8(data)?.to_string())
}
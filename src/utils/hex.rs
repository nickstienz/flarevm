use crate::components::error::Error;
use std::ops::{BitOrAssign, ShlAssign};

pub fn to_int<T>(bytes: &[u8]) -> T
where
    T: ShlAssign + BitOrAssign + From<u8> + Copy,
{
    let size = std::mem::size_of::<T>();

    if bytes.len() < size {
        Error::panic(
            Error::SliceTooSmall,
            format!("Bytes ({}) < Size ({})", bytes.len(), size),
        );
    }

    if bytes.len() > size {
        Error::panic(
            Error::SliceTooBig,
            format!("Bytes ({}) > Size ({})", bytes.len(), size),
        );
    }

    let mut value = T::from(bytes[0]);

    for i in 1..size {
        value <<= 8.into();
        value |= T::from(bytes[i]);
    }

    value
}

pub fn to_str(bytes: &[u8]) -> &str {
    match std::str::from_utf8(bytes) {
        Ok(s) => &s,
        Err(e) => Error::panic(Error::ByteToString, e.to_string()),
    }
}

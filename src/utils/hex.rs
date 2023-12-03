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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn to_int_too_small() {
        let list: Vec<u8> = vec![0x01];
        to_int::<u64>(&list);
    }

    #[test]
    #[should_panic]
    fn to_int_too_big() {
        let list: Vec<u8> = vec![0x01, 0x01];
        to_int::<u8>(&list);
    }

    #[test]
    fn to_int_u32() {
        let list: Vec<u8> = vec![0x00, 0x00, 0x00, 0x01];
        let num = to_int::<u32>(&list);
        assert_eq!(num, 1u32);
    }

    #[test]
    fn to_str_a() {
        let list: Vec<u8> = vec![0x61];
        let s = to_str(&list);
        assert_eq!(s, "a");
    }

    #[test]
    fn to_str_abc() {
        let list: Vec<u8> = vec![0x61, 0x62, 0x63];
        let s = to_str(&list);
        assert_eq!(s, "abc");
    }

    #[test]
    #[should_panic]
    fn to_str_invalid_utf8() {
        let list: Vec<u8> = vec![0xFC, 0x53];
        to_str(&list);
    }
}

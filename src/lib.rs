// ISC License
//
// Copyright (c) 2015,2026 Stephen Seo
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

use std::{error::Error, fmt::Display};

pub mod c_ffi;

#[derive(Debug, Copy, Clone)]
pub enum B64Error {
    InvalidDataSize,
    DataSizeMismatch,
    OutputSizeMismatch,
    InternalError,
    InvalidData,
}

impl Display for B64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            B64Error::InvalidDataSize => f.write_str("Invalid Data Size"),
            B64Error::DataSizeMismatch => f.write_str("Data Size Mismatch"),
            B64Error::OutputSizeMismatch => f.write_str("Output Size Mismatch"),
            B64Error::InternalError => f.write_str("Internal Error"),
            B64Error::InvalidData => f.write_str("Invalid Data"),
        }
    }
}

impl Error for B64Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[test]
fn it_works() {
    let s = "The test string".to_string();
    let sbytes = s.into_bytes();

    let b64_result = data_to_base64(&sbytes, false).expect("Should be able to encode base64");
    assert_eq!(b64_result.len(), 20);
    let data_result = base64_to_data(&b64_result).expect("Should be able to decode base64");
    assert_eq!(data_result.len(), 15);

    assert_eq!(&sbytes, &data_result);

    let s = "DapperBase64/+Hi".to_string();
    let sbytes = s.into_bytes();
    let data_result = base64_to_data(&sbytes).expect("Should be able to decode base64");
    assert_eq!(data_result.len(), 12);
    let b64_result = data_to_base64(&data_result, false).expect("Should be able to encode base64");
    assert_eq!(b64_result.len(), 16);

    assert_eq!(&sbytes, &b64_result);
}

/// Returns a base64 encoded character based on the value given
/// Values must be within range 0..64
/// # Examples
///
/// ```
/// use base64::data_to_base64_map;
///
/// assert_eq!(b"N"[0], data_to_base64_map(13u8, false));
/// assert_eq!(b"/"[0], data_to_base64_map(63u8, false));
/// assert_eq!(b"_"[0], data_to_base64_map(63u8, true));
/// ```
pub fn data_to_base64_map(value: u8, url_safe: bool) -> u8 {
    static BASE64_ARRAY: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    static BASE64_ARRAY_URLSAFE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    if url_safe {
        BASE64_ARRAY_URLSAFE[value as usize]
    } else {
        BASE64_ARRAY[value as usize]
    }
}

/// Encodes data from a byte array into a base64 encoded byte array.
///
/// note that in the following example 121, 101, 112, and 115 are decimal
/// values for the string "yeps" in utf-8.
/// # Examples
///
/// ```
/// use base64::data_to_base64;
///
/// let data = [121u8, 101u8, 112u8, 115u8];
/// let b64_result = data_to_base64(&data, false).expect("Should be able to encode base64");
/// assert_eq!(8, b64_result.len());
/// assert_eq!(b"eWVwcw==", b64_result.as_slice());
///
/// let data = [121u8, 101u8, 112u8];
/// let b64_result = data_to_base64(&data, false).expect("Should be able to encode base64");
/// assert_eq!(4, b64_result.len());
/// assert_eq!(b"eWVw", b64_result.as_slice());
///
/// let data = [121u8, 101u8];
/// let b64_result = data_to_base64(&data, false).expect("Should be able to encode base64");
/// assert_eq!(4, b64_result.len());
/// assert_eq!(b"eWU=", b64_result.as_slice());
/// ```
pub fn data_to_base64(data: &[u8], url_safe: bool) -> Result<Vec<u8>, B64Error> {
    if data.is_empty() {
        return Err(B64Error::InvalidDataSize);
    }

    let base64_size: usize = ((data.len() - 1) / 3 + 1) * 4;

    let mut base64_iter: usize = 0;
    let mut prev = 0u8;
    let mut prev_iter: usize = 0;

    let mut base64_result: Vec<u8> = vec![0; base64_size];

    for (idx, byte) in data.iter().enumerate() {
        match idx % 3 {
            0 => {
                if base64_iter >= base64_size {
                    return Err(B64Error::DataSizeMismatch);
                }
                base64_result[base64_iter] = data_to_base64_map((byte & 0xFCu8) >> 2, url_safe);
                base64_iter += 1;
            }
            1 => {
                if base64_iter >= base64_size {
                    return Err(B64Error::DataSizeMismatch);
                }
                base64_result[base64_iter] =
                    data_to_base64_map(((prev << 4) & 0x30u8) | (byte >> 4), url_safe);
                base64_iter += 1;
            }
            2 => {
                if base64_iter >= base64_size {
                    return Err(B64Error::DataSizeMismatch);
                }
                base64_result[base64_iter] =
                    data_to_base64_map(((prev << 2) & 0x3Cu8) | ((byte & 0xC0u8) >> 6), url_safe);
                base64_iter += 1;

                if base64_iter >= base64_size {
                    return Err(B64Error::DataSizeMismatch);
                }
                base64_result[base64_iter] = data_to_base64_map(byte & 0x3Fu8, url_safe);
                base64_iter += 1;
            }
            _ => unreachable!(),
        }

        prev = *byte;
        prev_iter = idx;
    }

    match prev_iter % 3 {
        0 => {
            if base64_iter >= base64_size {
                return Err(B64Error::DataSizeMismatch);
            }
            base64_result[base64_iter] = data_to_base64_map((prev & 0x3u8) << 4, url_safe);
            base64_iter += 1;

            if base64_iter >= base64_size {
                return Err(B64Error::DataSizeMismatch);
            }
            base64_result[base64_iter] = 61u8;
            base64_iter += 1;

            if base64_iter >= base64_size {
                return Err(B64Error::DataSizeMismatch);
            }
            base64_result[base64_iter] = 61u8;
            base64_iter += 1;
        }
        1 => {
            if base64_iter >= base64_size {
                return Err(B64Error::DataSizeMismatch);
            }
            base64_result[base64_iter] = data_to_base64_map((prev & 0xFu8) << 2, url_safe);
            base64_iter += 1;

            if base64_iter >= base64_size {
                return Err(B64Error::DataSizeMismatch);
            }
            base64_result[base64_iter] = 61u8;
            base64_iter += 1;
        }
        2 => (),
        _ => unreachable!(),
    }

    if base64_iter != base64_size {
        return Err(B64Error::DataSizeMismatch);
    }

    Ok(base64_result)
}

/// Returns data based on the base64 encoded character given
/// Values must be within range A..Z, a..z, 0..9, +, -, /, _
/// # Examples
///
/// ```
/// use base64::base64_to_data_map;
///
/// assert_eq!(18u8, base64_to_data_map(83u8));
/// assert_eq!(63u8, base64_to_data_map(47u8));
/// assert_eq!(63u8, base64_to_data_map(95u8));
/// ```
pub fn base64_to_data_map(base64: u8) -> u8 {
    static DATA_ARRAY: [u8; 128] = [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 62, 255, 62, 255, 63, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 61, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
        14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 255, 255, 255, 255, 63, 255, 26, 27, 28,
        29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
        255, 255, 255, 255, 255,
    ];

    DATA_ARRAY[base64 as usize]
}

/// Decodes data from base64 encoded array of bytes.
///
/// # Examples
///
/// ```
/// use base64::base64_to_data;
///
/// let base64: &[u8; 8] = b"////////";
/// let data_result = base64_to_data(base64).expect("Should be able to decode base64");
/// assert_eq!(data_result.len(), 6);
/// let result: [u8; 6] = [255, 255, 255, 255, 255, 255];
/// assert_eq!(&result, data_result.as_slice());
///
/// let base64: &[u8; 4] = b"//==";
/// let data_result = base64_to_data(base64).expect("Should be able to decode base64");
/// assert_eq!(data_result.len(), 2);
/// let result: [u8; 2] = [255, 0xF0];
/// assert_eq!(&result, data_result.as_slice());
///
/// let base64: &[u8; 4] = b"///=";
/// let data_result = base64_to_data(base64).expect("Should be able to decode base64");
/// assert_eq!(data_result.len(), 3);
/// let result: [u8; 3] = [255, 255, 0xC0];
/// assert_eq!(&result, data_result.as_slice());
///
/// let base64: &[u8; 8] = b"Q2hhcHM=";
/// let data_result = base64_to_data(base64).expect("Should be able to decode base64");
/// assert_eq!(data_result.len(), 6);
/// let result: [u8; 6] = [67, 104, 97, 112, 115, 0];
/// assert_eq!(&result, data_result.as_slice());
/// ```
pub fn base64_to_data(base64: &[u8]) -> Result<Vec<u8>, B64Error> {
    if base64.is_empty() || !base64.len().is_multiple_of(4) {
        return Err(B64Error::InvalidDataSize);
    }

    let mut amount_of_padding: u8 = 0;
    {
        let mut base64_iter: usize = base64.len();
        while base64[base64_iter - 1] == 61u8 {
            amount_of_padding += 1;
            base64_iter -= 1;
        }
        if amount_of_padding > 2 {
            return Err(B64Error::InvalidData);
        }
    }

    let data_size: usize = (base64.len() / 4 - if amount_of_padding > 0 { 1 } else { 0 }) * 3
        + match amount_of_padding {
            0 => 0,
            1 => 3,
            2 => 2,
            _ => unreachable!(),
        };

    let mut data_result: Vec<u8> = vec![0; data_size];

    let mut prev: u8 = 255u8;
    let mut data_iter: usize = 0;

    for (idx, byte) in base64.iter().enumerate() {
        if *byte == 61u8 {
            break;
        }

        let temp_data = base64_to_data_map(*byte);
        if temp_data >= 64u8 {
            return Err(B64Error::InvalidData);
        }
        match idx % 4 {
            0 => (),
            1 => {
                if data_iter >= data_size {
                    return Err(B64Error::DataSizeMismatch);
                }
                data_result[data_iter] = (prev << 2) | (temp_data >> 4);
                data_iter += 1;
            }
            2 => {
                if data_iter >= data_size {
                    return Err(B64Error::DataSizeMismatch);
                }
                data_result[data_iter] = (prev << 4) | (temp_data >> 2);
                data_iter += 1;
            }
            3 => {
                if data_iter >= data_size {
                    return Err(B64Error::DataSizeMismatch);
                }
                data_result[data_iter] = (prev << 6) | temp_data;
                data_iter += 1;
            }
            _ => unreachable!(),
        }

        prev = temp_data;
    }

    match amount_of_padding {
        0 => (),
        1 => {
            if data_iter >= data_size {
                return Err(B64Error::DataSizeMismatch);
            }
            data_result[data_iter] = prev << 6;
            data_iter += 1;
        }
        2 => {
            if data_iter >= data_size {
                return Err(B64Error::DataSizeMismatch);
            }
            data_result[data_iter] = prev << 4;
            data_iter += 1;
        }
        _ => unreachable!(),
    }

    if data_iter != data_size {
        return Err(B64Error::DataSizeMismatch);
    }

    Ok(data_result)
}

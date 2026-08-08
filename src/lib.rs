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

pub mod c_ffi;

#[test]
fn it_works() {
    let s = "The test string".to_string();
    let sbytes = s.into_bytes();
    let mut data: [u8; 15] = [0u8; 15];
    let mut base64: [u8; 20] = [0u8; 20];

    let result_size = data_to_base64(&sbytes, sbytes.len(), &mut base64, false);
    assert_eq!(result_size, base64.len());
    let result_size = base64_to_data(&base64, base64.len(), &mut data);
    assert_eq!(result_size, data.len());

    assert_eq!(sbytes, data);

    let s = "DapperBase64/+Hi".to_string();
    let sbytes = s.into_bytes();
    let mut data: [u8; 12] = [0u8; 12];
    let mut base64: [u8; 16] = [0u8; 16];
    let result_size = base64_to_data(&sbytes, sbytes.len(), &mut data);
    assert_eq!(result_size, data.len());
    let result_size = data_to_base64(&data, data.len(), &mut base64, false);
    assert_eq!(result_size, base64.len());

    assert_eq!(sbytes, base64);
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
    static BASE64_ARRAY: &'static [u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    static BASE64_ARRAY_URLSAFE: &'static [u8; 64] =
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
/// let mut base64_array = [0u8; 8];
/// let mut base64_size = data_to_base64(&data, data.len(), &mut base64_array, false);
/// assert_eq!(8, base64_size);
/// assert_eq!(b"eWVwcw==", &base64_array);
///
/// let data = [121u8, 101u8, 112u8];
/// let mut base64_array = [0u8; 4];
/// base64_size = data_to_base64(&data, data.len(), &mut base64_array, false);
/// assert_eq!(4, base64_size);
/// assert_eq!(b"eWVw", &base64_array);
///
/// let data = [121u8, 101u8];
/// base64_size = data_to_base64(&data, data.len(), &mut base64_array, false);
/// assert_eq!(4, base64_size);
/// assert_eq!(b"eWU=", &base64_array);
/// ```
pub fn data_to_base64(
    data: &[u8],
    data_size: usize,
    base64_result: &mut [u8],
    url_safe: bool,
) -> usize {
    if data_size == 0 {
        panic!("ERROR: Given data size is zero!");
    } else if data_size > data.len() {
        panic!("ERROR: Given data size is greater than the data array!");
    }

    let base64_size: usize = ((data.len() - 1) / 3 + 1) * 4;

    if base64_size > base64_result.len() {
        panic!(
            "ERROR: calculated resulting size of data to base64 conversion is bigger than base64_result array!"
        );
    }

    let mut base64_iter: usize = 0;
    let mut prev = 0u8;
    let mut prev_iter: usize = 0;

    for i in 0..data_size {
        match i % 3 {
            0 => {
                if base64_iter >= base64_size {
                    panic!("base64_result index is greater than or equal to calculated size!");
                }
                base64_result[base64_iter] = data_to_base64_map((data[i] & 0xFCu8) >> 2, url_safe);
                base64_iter += 1;
            }
            1 => {
                if base64_iter >= base64_size {
                    panic!("base64_result index is greater than or equal to calculated size!");
                }
                base64_result[base64_iter] =
                    data_to_base64_map(((prev << 4) & 0x30u8) | (data[i] >> 4), url_safe);
                base64_iter += 1;
            }
            2 => {
                if base64_iter >= base64_size {
                    panic!("base64_result index is greater than or equal to calculated size!");
                }
                base64_result[base64_iter] = data_to_base64_map(
                    ((prev << 2) & 0x3Cu8) | ((data[i] & 0xC0u8) >> 6),
                    url_safe,
                );
                base64_iter += 1;

                if base64_iter >= base64_size {
                    panic!("base64_result index is greater than or equal to calculated size!");
                }
                base64_result[base64_iter] = data_to_base64_map(data[i] & 0x3Fu8, url_safe);
                base64_iter += 1;
            }
            _ => unreachable!(),
        }

        prev = data[i];
        prev_iter = i;
    }

    match prev_iter % 3 {
        0 => {
            if base64_iter >= base64_size {
                panic!("base64_result index is greater than or equal to calculated size!");
            }
            base64_result[base64_iter] = data_to_base64_map((prev & 0x3u8) << 4, url_safe);
            base64_iter += 1;

            if base64_iter >= base64_size {
                panic!("base64_result index is greater than or equal to calculated size!");
            }
            base64_result[base64_iter] = 61u8;
            base64_iter += 1;

            if base64_iter >= base64_size {
                panic!("base64_result index is greater than or equal to calculated size!");
            }
            base64_result[base64_iter] = 61u8;
            base64_iter += 1;
        }
        1 => {
            if base64_iter >= base64_size {
                panic!("base64_result index is greater than or equal to calculated size!");
            }
            base64_result[base64_iter] = data_to_base64_map((prev & 0xFu8) << 2, url_safe);
            base64_iter += 1;

            if base64_iter >= base64_size {
                panic!("base64_result index is greater than or equal to calculated size!");
            }
            base64_result[base64_iter] = 61u8;
            base64_iter += 1;
        }
        2 => (),
        _ => unreachable!(),
    }

    if base64_iter != base64_size {
        panic!(
            "ERROR: Function ended with incorrect final base64_iter value of {}; base64_size is {}",
            base64_iter, base64_size
        );
    }

    base64_size
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
/// let mut data: [u8; 6] = [0u8; 6];
/// let mut data_size: usize = base64_to_data( base64, base64.len(), &mut data );
/// assert_eq!(data_size, 6);
/// let result: [u8; 6] = [255, 255, 255, 255, 255, 255];
/// assert_eq!(&result, &data);
///
/// let base64: &[u8; 4] = b"//==";
/// data[2] = 0;
/// data[3] = 0;
/// data[4] = 0;
/// data[5] = 0;
/// data_size = base64_to_data(base64, base64.len(), &mut data);
/// assert_eq!(data_size, 2);
/// let result: [u8; 6] = [255, 0xF0, 0, 0, 0, 0];
/// assert_eq!(&result, &data);
///
/// let base64: &[u8; 4] = b"///=";
/// data[3] = 0;
/// data[4] = 0;
/// data[5] = 0;
/// data_size = base64_to_data(base64, base64.len(), &mut data);
/// assert_eq!(data_size, 3);
/// let result: [u8; 6] = [255, 255, 0xC0, 0, 0, 0];
/// assert_eq!(&result, &data);
///
/// let base64: &[u8; 8] = b"Q2hhcHM=";
/// data_size = base64_to_data(base64, base64.len(), &mut data);
/// assert_eq!(data_size, 6);
/// let result: [u8; 6] = [67, 104, 97, 112, 115, 0];
/// assert_eq!(&result, &data);
/// ```
pub fn base64_to_data(base64: &[u8], base64_size: usize, data_result: &mut [u8]) -> usize {
    if base64_size == 0 {
        panic!("ERROR: Given base64_size is zero!");
    } else if base64_size > base64.len() {
        panic!("ERROR: Given base64 size is greater than the base64 array!");
    } else if base64_size % 4 != 0 {
        panic!("ERROR: base64 array is of irregular length not divisible by 4!");
    }

    let mut amount_of_padding: u8 = 0;
    {
        let mut base64_iter: usize = base64_size;
        while base64[base64_iter - 1] == 61u8 {
            amount_of_padding += 1;
            base64_iter -= 1;
        }
        if amount_of_padding > 2 {
            panic!("ERROR: Invalid amount of padding!");
        }
    }

    let data_size: usize = (base64_size / 4 - if amount_of_padding > 0 { 1 } else { 0 }) * 3
        + match amount_of_padding {
            0 => 0,
            1 => 3,
            2 => 2,
            _ => unreachable!(),
        };

    if data_size > data_result.len() {
        panic!(
            "ERROR: calculated resulting size of base64 to data conversion is bigger than data_result array!"
        );
    }

    let mut prev: u8 = 255u8;
    let mut data_iter: usize = 0;

    for i in 0..base64_size {
        if base64[i] == 61u8 {
            break;
        }

        let temp_data = base64_to_data_map(base64[i]);
        if temp_data >= 64u8 {
            panic!("ERROR: Invalid byte in base64 array!")
        }
        match i % 4 {
            0 => (),
            1 => {
                if data_iter >= data_size {
                    panic!("ERROR: data_iter is greater than or equal to data_size!");
                }
                data_result[data_iter] = (prev << 2) | (temp_data >> 4);
                data_iter += 1;
            }
            2 => {
                if data_iter >= data_size {
                    panic!("ERROR: data_iter is greater than or equal to data_size!");
                }
                data_result[data_iter] = (prev << 4) | (temp_data >> 2);
                data_iter += 1;
            }
            3 => {
                if data_iter >= data_size {
                    panic!("ERROR: data_iter is greater than or equal to data_size!");
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
                panic!("ERROR: data_iter is greater than or equal to data_size!");
            }
            data_result[data_iter] = prev << 6;
            data_iter += 1;
        }
        2 => {
            if data_iter >= data_size {
                panic!("ERROR: data_iter is greater than or equal to data_size!");
            }
            data_result[data_iter] = prev << 4;
            data_iter += 1;
        }
        _ => unreachable!(),
    }

    if data_iter != data_size {
        panic!(
            "ERROR: Function ended with incorrect final data_iter value of {}; data_size is {}",
            data_iter, data_size
        );
    }

    data_size
}

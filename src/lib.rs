use std::string::String;
use std::char;

#[test]
fn it_works() {
//    let s = "The test string".to_string();

    let byte: u8 = 0xFF;
    assert_eq!(byte & 0xFC, 0xFC);

    assert_eq!(121u8, 0x79u8);

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
pub fn data_to_base64_map( value: u8, url_safe: bool ) -> u8 {
    static BASE64_ARRAY: &'static [u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    static BASE64_ARRAY_URLSAFE: &'static [u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    if url_safe {
        BASE64_ARRAY_URLSAFE[value as usize]
    } else {
        BASE64_ARRAY[value as usize]
    }
}

/// Encodes data into a base64 encoded string.
///
/// note that in the following example 121, 101, 112, and 115 are decimal
/// values for the string "yeps" in utf-8.
/// # Examples
///
/// ```
/// use base64::data_to_base64;
///
/// let data = [121u8, 101u8, 112u8, 115u8];
/// let base64 = data_to_base64(&data, false);
/// assert_eq!("eWVwcw==", base64);
///
/// let data = [121u8, 101u8, 112u8];
/// let base64 = data_to_base64(&data, false);
/// assert_eq!("eWVw", base64);
///
/// let data = [121u8, 101u8];
/// let base64 = data_to_base64(&data, false);
/// assert_eq!("eWU=", base64);
/// ```
pub fn data_to_base64( data: &[u8], url_safe: bool ) -> String {
    if data.len() == 0 {
        return String::new();
    }

    let mut prev = 0u8;
    let mut prev_iter: usize = 0;
    let mut base64: String = String::new();

    for i in 0..data.len() {
        match i % 3 {
            0 => {
                let character = char::from_u32(data_to_base64_map((data[i] & 0xFCu8) >> 2, url_safe) as u32);
                match character {
                    Some(a_character) => base64.push(a_character),
                    _ => panic!("Failed to convert byte to character!"),
                }
            },
            1 => {
                let character = char::from_u32(data_to_base64_map(((prev << 4) & 0x30u8) | (data[i] >> 4), url_safe) as u32);
                match character {
                    Some(a_character) => base64.push(a_character),
                    _ => panic!("Failed to convert byte to character!"),
                }
            },
            2 => {
                let character = char::from_u32(data_to_base64_map(((prev << 2) & 0x3Cu8) | ((data[i] & 0xC0u8) >> 6), url_safe) as u32);
                match character {
                    Some(a_character) => base64.push(a_character),
                    _ => panic!("Failed to convert byte to character!"),
                }

                let character = char::from_u32(data_to_base64_map(data[i] & 0x3Fu8, url_safe) as u32);
                match character {
                    Some(a_character) => base64.push(a_character),
                    _ => panic!("Failed to convert byte to character!"),
                }
            },
            _ => unreachable!(),
        }

        prev = data[i];
        prev_iter = i;
    }

    match prev_iter % 3 {
        0 => {
            let character = char::from_u32(data_to_base64_map((prev & 0x3u8) << 4, url_safe) as u32);
            match character {
                Some(a_character) => base64.push(a_character),
                _ => panic!("Failed to convert byte to character!"),
            }
            base64.push_str("==");
        },
        1 => {
            let character = char::from_u32(data_to_base64_map((prev & 0xFu8) << 2, url_safe) as u32);
            match character {
                Some(a_character) => base64.push(a_character),
                _ => panic!("Failed to convert byte to character!"),
            }
            base64.push_str("=");
        },
        2 => (),
        _ => unreachable!(),
    }

    base64
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
pub fn base64_to_data_map( base64: u8 ) -> u8 {
    static DATA_ARRAY: [u8; 128] =
        [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,  62, 255,  62, 255, 63,
          52,  53,  54,  55,  56,  57,  58,  59,  60,  61, 255, 255, 255, 255, 255, 255,
         255,   0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
          15,  16,  17,  18,  19,  20,  21,  22,  23,  24,  25, 255, 255, 255, 255,  63,
         255,  26,  27,  28,  29,  30,  31,  32,  33,  34,  35,  36,  37,  38,  39,  40,
          41,  42,  43,  44,  45,  46,  47,  48,  49,  50,  51, 255, 255, 255, 255, 255];

    DATA_ARRAY[base64 as usize]
}


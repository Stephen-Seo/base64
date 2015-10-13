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
/// assert_eq!(18u8, base64_to_data_map("S"));
/// assert_eq!(63u8, base64_to_data_map("/"));
/// assert_eq!(63u8, base64_to_data_map("_"));
/// ```
pub fn base64_to_data_map( base64: &str ) -> u8 {
    match base64 {
        "A" => 0u8,
        "B" => 1u8,
        "C" => 2u8,
        "D" => 3u8,
        "E" => 4u8,
        "F" => 5u8,
        "G" => 6u8,
        "H" => 7u8,
        "I" => 8u8,
        "J" => 9u8,
        "K" => 10u8,
        "L" => 11u8,
        "M" => 12u8,
        "N" => 13u8,
        "O" => 14u8,
        "P" => 15u8,
        "Q" => 16u8,
        "R" => 17u8,
        "S" => 18u8,
        "T" => 19u8,
        "U" => 20u8,
        "V" => 21u8,
        "W" => 22u8,
        "X" => 23u8,
        "Y" => 24u8,
        "Z" => 25u8,
        "a" => 26u8,
        "b" => 27u8,
        "c" => 28u8,
        "d" => 29u8,
        "e" => 30u8,
        "f" => 31u8,
        "g" => 32u8,
        "h" => 33u8,
        "i" => 34u8,
        "j" => 35u8,
        "k" => 36u8,
        "l" => 37u8,
        "m" => 38u8,
        "n" => 39u8,
        "o" => 40u8,
        "p" => 41u8,
        "q" => 42u8,
        "r" => 43u8,
        "s" => 44u8,
        "t" => 45u8,
        "u" => 46u8,
        "v" => 47u8,
        "w" => 48u8,
        "x" => 49u8,
        "y" => 50u8,
        "z" => 51u8,
        "0" => 52u8,
        "1" => 53u8,
        "2" => 54u8,
        "3" => 55u8,
        "4" => 56u8,
        "5" => 57u8,
        "6" => 58u8,
        "7" => 59u8,
        "8" => 60u8,
        "9" => 61u8,
        "+" => 62u8,
        "-" => 62u8,
        "/" => 63u8,
        "_" => 63u8,
        _ => panic!("Error: value is out of range! (value is {})", base64),
    }
}



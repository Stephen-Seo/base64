

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
/// use base64::base64_map;
///
/// assert_eq!("N".to_string(), base64_map(13u8, false));
/// assert_eq!("/".to_string(), base64_map(63u8, false));
/// assert_eq!("_".to_string(), base64_map(63u8, true));
/// ```
pub fn base64_map( value: u8, url_safe: bool ) -> String {
    match value {
        0 => "A".to_string(),
        1 => "B".to_string(),
        2 => "C".to_string(),
        3 => "D".to_string(),
        4 => "E".to_string(),
        5 => "F".to_string(),
        6 => "G".to_string(),
        7 => "H".to_string(),
        8 => "I".to_string(),
        9 => "J".to_string(),
        10 => "K".to_string(),
        11 => "L".to_string(),
        12 => "M".to_string(),
        13 => "N".to_string(),
        14 => "O".to_string(),
        15 => "P".to_string(),
        16 => "Q".to_string(),
        17 => "R".to_string(),
        18 => "S".to_string(),
        19 => "T".to_string(),
        20 => "U".to_string(),
        21 => "V".to_string(),
        22 => "W".to_string(),
        23 => "X".to_string(),
        24 => "Y".to_string(),
        25 => "Z".to_string(),
        26 => "a".to_string(),
        27 => "b".to_string(),
        28 => "c".to_string(),
        29 => "d".to_string(),
        30 => "e".to_string(),
        31 => "f".to_string(),
        32 => "g".to_string(),
        33 => "h".to_string(),
        34 => "i".to_string(),
        35 => "j".to_string(),
        36 => "k".to_string(),
        37 => "l".to_string(),
        38 => "m".to_string(),
        39 => "n".to_string(),
        40 => "o".to_string(),
        41 => "p".to_string(),
        42 => "q".to_string(),
        43 => "r".to_string(),
        44 => "s".to_string(),
        45 => "t".to_string(),
        46 => "u".to_string(),
        47 => "v".to_string(),
        48 => "w".to_string(),
        49 => "x".to_string(),
        50 => "y".to_string(),
        51 => "z".to_string(),
        52 => "0".to_string(),
        53 => "1".to_string(),
        54 => "2".to_string(),
        55 => "3".to_string(),
        56 => "4".to_string(),
        57 => "5".to_string(),
        58 => "6".to_string(),
        59 => "7".to_string(),
        60 => "8".to_string(),
        61 => "9".to_string(),
        62 if !url_safe => "+".to_string(),
        62 if url_safe => "-".to_string(),
        63 if !url_safe => "/".to_string(),
        63 if url_safe => "_".to_string(),
        _ => panic!("Error: value is out of range! (value is {})", value),
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
/// let mut data = vec![121u8, 101u8, 112u8, 115u8];
/// let base64 = data_to_base64(&data, false);
/// assert_eq!("eWVwcw==", base64);
///
/// data.pop();
/// let base64 = data_to_base64(&data, false);
/// assert_eq!("eWVw", base64);
///
/// data.pop();
/// let base64 = data_to_base64(&data, false);
/// assert_eq!("eWU=", base64);
/// ```
pub fn data_to_base64( data: &Vec<u8>, url_safe: bool ) -> String {
    let mut prev = 0u8;
    let mut prev_iter: usize = 0;
    let mut base64 = String::new();
    for (iter, current) in data.iter().enumerate() {
        if iter % 3 == 0 {
            base64.push_str(&base64_map((current & 0xFCu8) >> 2, url_safe));
        } else if iter % 3 == 1 {
            base64.push_str(&base64_map(((prev << 4) & 0x30u8) | (current >> 4), url_safe));
        } else if iter % 3 == 2 {
            base64.push_str(&base64_map(((prev << 2) & 0x3Cu8) | ((current & 0xC0u8) >> 6), url_safe));
            base64.push_str(&base64_map(current & 0x3Fu8, url_safe));
        }

        prev = *current;
        prev_iter = iter;
    }

    if prev_iter % 3 == 0 {
        base64.push_str(&base64_map((prev & 0x3u8) << 4, url_safe));
        base64.push_str("==");
    } else if prev_iter % 3 == 1 {
        base64.push_str(&base64_map((prev & 0xFu8) << 2, url_safe));
        base64.push_str("=");
    }

    base64
}


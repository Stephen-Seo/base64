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

use core::ffi as c_ffi;
use core::ptr as c_ptr;
use core::slice;

use crate::base64_to_data;
use crate::data_to_base64;

use libc::{free, malloc};

/// C-interface to Rust function data_to_base64.
///
/// ```
/// use core::ffi as c_ffi;
/// use core::ptr as c_ptr;
/// use core::slice;
///
/// use base64::c_ffi::data_to_base64_c_interface;
///
/// use libc::free;
///
/// let data = [121u8, 101u8, 112u8, 115u8];
/// let mut b64_size_out: c_ffi::c_ulonglong = 0;
/// unsafe {
///   let mut encoded: *mut c_ffi::c_char = data_to_base64_c_interface(&data as *const u8 as *const c_ffi::c_void, data.len() as c_ffi::c_ulonglong, 0, (&mut b64_size_out) as *mut c_ffi::c_ulonglong, 1);
///   assert_eq!(8, b64_size_out);
///   let b64_slice: &[u8] = slice::from_raw_parts(encoded as *const u8, 8);
///   assert_eq!(b"eWVwcw==", b64_slice);
///   assert_eq!(*encoded.byte_offset(8), 0);
///   free(encoded as *mut c_ffi::c_void);
/// }
///
/// let data = [121u8, 101u8, 112u8];
/// unsafe {
///   let mut encoded: *mut c_ffi::c_char = data_to_base64_c_interface(&data as *const u8 as *const c_ffi::c_void, data.len() as c_ffi::c_ulonglong, 0, (&mut b64_size_out) as *mut c_ffi::c_ulonglong, 1);
///   assert_eq!(4, b64_size_out);
///   let b64_slice: &[u8] = slice::from_raw_parts(encoded as *const u8, 4);
///   assert_eq!(b"eWVw", b64_slice);
///   assert_eq!(*encoded.byte_offset(4), 0);
///   free(encoded as *mut c_ffi::c_void);
/// }
///
/// let data = [121u8, 101u8];
/// unsafe {
///   let mut encoded: *mut c_ffi::c_char = data_to_base64_c_interface(&data as *const u8 as *const c_ffi::c_void, data.len() as c_ffi::c_ulonglong, 0, (&mut b64_size_out) as *mut c_ffi::c_ulonglong, 1);
///   assert_eq!(4, b64_size_out);
///   let b64_slice: &[u8] = slice::from_raw_parts(encoded as *const u8, 4);
///   assert_eq!(b"eWU=", b64_slice);
///   assert_eq!(*encoded.byte_offset(4), 0);
///   free(encoded as *mut c_ffi::c_void);
/// }
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn data_to_base64_c_interface(
    data: *const c_ffi::c_void,
    data_size: c_ffi::c_ulonglong,
    url_safe: c_ffi::c_int,
    b64_size_out: *mut c_ffi::c_ulonglong,
    stderr_on_error: c_ffi::c_int,
) -> *mut c_ffi::c_char {
    if data.is_null() || b64_size_out.is_null() {
        if stderr_on_error != 0 {
            eprintln!("NULL passed to data_to_base64_c_interface!");
        }
        return c_ptr::null_mut();
    }

    let b64_mod: c_ffi::c_ulonglong = data_size % 3;
    let b64_len: c_ffi::c_ulonglong = (data_size / 3 + if b64_mod != 0 { 1 } else { 0 }) * 4;

    unsafe {
        (*b64_size_out) = b64_len;
        let data_slice: &[u8] = slice::from_raw_parts(data as *const u8, data_size as usize);
        // +1 for the NULL terminator.
        let malloced_data: *mut c_ffi::c_void = malloc(b64_len as libc::size_t + 1);

        let b64_result = data_to_base64(data_slice, if url_safe == 0 { false } else { true });

        if let Err(e) = b64_result {
            if stderr_on_error != 0 {
                eprintln!("Failed to encode to base64: {}!", e);
            }

            free(malloced_data);
            return c_ptr::null_mut();
        }

        let b64 = b64_result.unwrap();

        if b64.len() != b64_len as usize {
            if stderr_on_error != 0 {
                eprintln!("Failed to encode to base64: encoded size mismatch!");
            }
            free(malloced_data);
            return c_ptr::null_mut();
        }

        let malloced_slice: &mut [u8] =
            slice::from_raw_parts_mut(malloced_data as *mut u8, b64_len as usize);

        // Shouldn't panic due to previous check on sizes.
        for (idx, byte) in b64.into_iter().enumerate() {
            malloced_slice[idx] = byte;
        }

        // Apply the NULL terminator.
        *(malloced_data.byte_offset(b64_len as isize) as *mut c_ffi::c_char) = 0;

        return malloced_data as *mut c_ffi::c_char;
    }
}

/// ```
/// use core::ffi as c_ffi;
/// use core::ptr as c_ptr;
/// use core::slice;
///
/// use base64::c_ffi::base64_to_data_c_interface;
///
/// use libc::free;
///
/// let base64: &[u8; 8] = b"////////";
/// let mut data_size_out: c_ffi::c_ulonglong = 0;
/// unsafe {
///   let mut decoded: *mut c_ffi::c_void = base64_to_data_c_interface(base64 as *const u8 as *const c_ffi::c_char, base64.len() as c_ffi::c_ulonglong, (&mut data_size_out) as *mut c_ffi::c_ulonglong, 1);
///   assert!(!decoded.is_null());
///   assert_eq!(data_size_out, 6);
///   let result: [u8; 6] = [255, 255, 255, 255, 255, 255];
///   let data: &[u8] = slice::from_raw_parts(decoded as *const u8, 6);
///   assert_eq!(&result, data);
///   assert_eq!(*(decoded.byte_offset(6) as *const u8), 0);
///   free(decoded);
/// }
///
/// let base64: &[u8; 4] = b"//==";
/// data_size_out = 0;
/// unsafe {
///   let mut decoded: *mut c_ffi::c_void = base64_to_data_c_interface(base64 as *const u8 as *const c_ffi::c_char, base64.len() as c_ffi::c_ulonglong, (&mut data_size_out) as *mut c_ffi::c_ulonglong, 1);
///   assert!(!decoded.is_null());
///   assert_eq!(data_size_out, 2);
///   let result: [u8; 2] = [255, 0xF0];
///   let data: &[u8] = slice::from_raw_parts(decoded as *const u8, 2);
///   assert_eq!(&result, data);
///   assert_eq!(*(decoded.byte_offset(2) as *const u8), 0);
///   free(decoded);
/// }
///
/// let base64: &[u8; 4] = b"///=";
/// data_size_out = 0;
/// unsafe {
///   let mut decoded: *mut c_ffi::c_void = base64_to_data_c_interface(base64 as *const u8 as *const c_ffi::c_char, base64.len() as c_ffi::c_ulonglong, (&mut data_size_out) as *mut c_ffi::c_ulonglong, 1);
///   assert!(!decoded.is_null());
///   assert_eq!(data_size_out, 3);
///   let result: [u8; 3] = [255, 255, 0xC0];
///   let data: &[u8] = slice::from_raw_parts(decoded as *const u8, 3);
///   assert_eq!(&result, data);
///   assert_eq!(*(decoded.byte_offset(3) as *const u8), 0);
///   free(decoded);
/// }
///
/// let base64: &[u8; 8] = b"Q2hhcHM=";
/// data_size_out = 0;
/// unsafe {
///   let mut decoded: *mut c_ffi::c_void = base64_to_data_c_interface(base64 as *const u8 as *const c_ffi::c_char, base64.len() as c_ffi::c_ulonglong, (&mut data_size_out) as *mut c_ffi::c_ulonglong, 1);
///   assert!(!decoded.is_null());
///   assert_eq!(data_size_out, 6);
///   let result: [u8; 6] = [67, 104, 97, 112, 115, 0];
///   let data: &[u8] = slice::from_raw_parts(decoded as *const u8, 6);
///   assert_eq!(&result, data);
///   assert_eq!(*(decoded.byte_offset(6) as *const u8), 0);
///   free(decoded);
/// }
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn base64_to_data_c_interface(
    b64: *const c_ffi::c_char,
    b64_size: c_ffi::c_ulonglong,
    data_size_out: *mut c_ffi::c_ulonglong,
    stderr_on_error: c_ffi::c_int,
) -> *mut c_ffi::c_void {
    if b64.is_null() || data_size_out.is_null() {
        if stderr_on_error != 0 {
            eprintln!("NULL passed to base64_to_data_c_interface!");
        }
        return c_ptr::null_mut();
    }

    let mut padding_count: u8 = 0;
    unsafe {
        let mut idx = b64_size;
        let mut b64_ptr: *const c_ffi::c_char = b64.wrapping_byte_add(b64_size as usize - 1);
        while (*b64_ptr) == 0x3D {
            padding_count += 1;
            idx -= 1;
            if padding_count > 2 || idx == 0 {
                if stderr_on_error != 0 {
                    eprintln!("Internal error in base64_to_data_c_interface!");
                }
                return c_ptr::null_mut();
            }
            b64_ptr = b64_ptr.wrapping_byte_sub(1);
        }
    }

    let data_len: c_ffi::c_ulonglong = (b64_size / 4 - if padding_count > 0 { 1 } else { 0 }) * 3
        + match padding_count {
            0 => 0,
            1 => 3,
            2 => 2,
            _ => unreachable!(),
        };

    unsafe {
        (*data_size_out) = data_len;
        let b64_slice: &[u8] = slice::from_raw_parts(b64 as *const u8, b64_size as usize);
        // +1 for the NULL terminator.
        let malloced_data: *mut c_ffi::c_void = malloc(data_len as libc::size_t + 1);

        let data_result = base64_to_data(b64_slice);

        if let Err(e) = data_result {
            if stderr_on_error != 0 {
                eprintln!("Failed to decode from base64: {}!", e);
            }

            free(malloced_data);
            return c_ptr::null_mut();
        }

        let data = data_result.unwrap();

        if data.len() != data_len as usize {
            if stderr_on_error != 0 {
                eprintln!("Failed to decode from base64: decoded size mismatch!");
            }
            free(malloced_data);
            return c_ptr::null_mut();
        }

        let malloced_slice: &mut [u8] =
            slice::from_raw_parts_mut(malloced_data as *mut u8, data_len as usize);

        // Shouldn't panic due to previous check on sizes.
        for (idx, byte) in data.into_iter().enumerate() {
            malloced_slice[idx] = byte;
        }

        // Apply the NULL terminator.
        *(malloced_data.byte_offset(data_len as isize) as *mut c_ffi::c_char) = 0;

        return malloced_data;
    }
}

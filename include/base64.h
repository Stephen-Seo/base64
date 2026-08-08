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

#ifndef COM_SEODISPARATE_RUST_C_LIB_BASE_64_H_
#define COM_SEODISPARATE_RUST_C_LIB_BASE_64_H_

#ifdef __cplusplus
extern "C" {
#endif

// NULL on error, must be FREE'd after use.
char *data_to_base64_c_interface(const void *data, unsigned long long data_size, int url_safe, unsigned long long *b64_size_out);

// NULL on error, must be FREE'd after use.
void *base64_to_data_c_interface(const char *base64, unsigned long long base64_size, unsigned long long *data_size_out);

#ifdef __cplusplus
}
#endif

#endif

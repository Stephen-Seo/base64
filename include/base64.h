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

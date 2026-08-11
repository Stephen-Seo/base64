// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <https://unlicense.org>

#include <base64.h>

#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int main(void) {
    const char *data = "apple";
    unsigned long long size = 0;
    char *b64 = data_to_base64_c_interface(data, 5, 0, &size, 1);

    printf("size: %llu\n", size);
    if (b64) {
        printf("%s\n", b64);

        unsigned long long data_size = 0;
        char *data_again = base64_to_data_c_interface(b64, size, &data_size, 1);

        printf("data_size: %llu\n", data_size);
        if (data_again) {
            printf("%.*s\n", data_size, data_again);
            free(data_again);
        }

        free(b64);
    }

    return 0;
}

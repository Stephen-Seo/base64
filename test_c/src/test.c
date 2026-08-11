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
        free(b64);
    }

    return 0;
}

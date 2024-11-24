extern const char* plitedb_version();

#include <stdio.h>
#include <string.h>

int main(void) {
    const char* version = plitedb_version();
    printf("Version: %s (len: %ld)\n", version, strlen(version));

    return 0;
}

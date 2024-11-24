extern long long convert_error();

#include <stdio.h>

int main(void) {
    const long long error_code = convert_error();

    printf("%lld\n", error_code);

    return 0;
}

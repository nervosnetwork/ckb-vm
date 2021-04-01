#include <stdint.h>

uint8_t buffer[4096] __attribute__((aligned(4096))) = {1, 2};

int main() {
    return buffer[0] + buffer[1] - 3;
}

// Build: /opt/riscv64b/bin/riscv64-unknown-elf-gcc -o pcnt -march=rv64gb pcnt.c

#include "rvintrin.h"
#include <stdint.h>

// Derived from
// https://github.com/FFmpeg/FFmpeg/blob/master/libavutil/common.h#L454-L461
static int popcnt32(uint32_t x) {
  x -= (x >> 1) & 0x55555555;
  x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
  x = (x + (x >> 4)) & 0x0F0F0F0F;
  x += x >> 8;
  return (x + (x >> 16)) & 0x3F;
}

static int popcnt64(uint64_t x) {
  return popcnt32((uint32_t)x) + popcnt32((uint32_t)(x >> 32));
}

// Test cases are copied from
// https://github.com/gcc-mirror/gcc/blob/16e2427f50c208dfe07d07f18009969502c25dc8/gcc/testsuite/gdc.test/runnable/builtin.d#L83-L104
#define CASES 13
uint64_t test[CASES] = {
    0,
    7,
    0xAA,
    0xFFFF,
    0xCCCC,
    0x7777,
    0x84211248,
    0xFFFFFFFF,
    0xCCCCCCCC,
    0x77777777,
    0xFFFFFFFFFFFFFFFF,
    0xCCCCCCCCCCCCCCCC,
    0x7777777777777777,
};

int main() {
  for (int i = 0; i < CASES; i++) {
    uint64_t n = test[i];
    int a = popcnt64(n);
    int b = _rv64_pcnt(n);
    if (a != b) {
      return 1;
    }
    int c = popcnt32((uint32_t)n);
    int d = _rv32_pcnt((uint32_t)n);
    if (c != d) {
      return 1;
    }
  }
  return 0;
}

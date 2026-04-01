#include <stdint.h>

static int failed = 0;

static void check_div(int64_t a, int64_t b, int64_t expected) {
    int64_t result;
    asm volatile("div %0, %1, %2" : "=r"(result) : "r"(a), "r"(b));
    if (result != expected) {
        failed = 1;
    }
}

static void check_divu(uint64_t a, uint64_t b, uint64_t expected) {
    uint64_t result;
    asm volatile("divu %0, %1, %2" : "=r"(result) : "r"(a), "r"(b));
    if (result != expected) {
        failed = 1;
    }
}

static void check_divw(int32_t a, int32_t b, int64_t expected) {
    int64_t result;
    asm volatile("divw %0, %1, %2" : "=r"(result) : "r"((int64_t)a), "r"((int64_t)b));
    if (result != expected) {
        failed = 1;
    }
}

static void check_divuw(uint32_t a, uint32_t b, int64_t expected) {
    int64_t result;
    asm volatile("divuw %0, %1, %2" : "=r"(result) : "r"((int64_t)a), "r"((int64_t)b));
    if (result != expected) {
        failed = 1;
    }
}

static void check_rem(int64_t a, int64_t b, int64_t expected) {
    int64_t result;
    asm volatile("rem %0, %1, %2" : "=r"(result) : "r"(a), "r"(b));
    if (result != expected) {
        failed = 1;
    }
}

static void check_remu(uint64_t a, uint64_t b, uint64_t expected) {
    uint64_t result;
    asm volatile("remu %0, %1, %2" : "=r"(result) : "r"(a), "r"(b));
    if (result != expected) {
        failed = 1;
    }
}

static void check_remw(int32_t a, int32_t b, int64_t expected) {
    int64_t result;
    asm volatile("remw %0, %1, %2" : "=r"(result) : "r"((int64_t)a), "r"((int64_t)b));
    if (result != expected) {
        failed = 1;
    }
}

static void check_remuw(uint32_t a, uint32_t b, int64_t expected) {
    int64_t result;
    asm volatile("remuw %0, %1, %2" : "=r"(result) : "r"((int64_t)a), "r"((int64_t)b));
    if (result != expected) {
        failed = 1;
    }
}

int main() {
    /* div/divu */
    check_div(10, 3, 3);
    check_div(-10, 3, -3);
    check_div(10, -3, -3);
    check_div(-10, -3, 3);
    check_div(42, 0, -1);
    check_div(INT64_MIN, -1, INT64_MIN);

    check_divu(10, 3, 3);
    check_divu(0, 5, 0);
    check_divu(42, 0, UINT64_MAX);
    check_divu(0x8000000000000000ULL, 2, 0x4000000000000000ULL);

    /* divw/divuw */
    check_divw(10, 3, 3);
    check_divw(-10, 3, -3);
    check_divw(10, -3, -3);
    check_divw(-10, -3, 3);
    check_divw(42, 0, -1);
    check_divw(-1, 0, -1);
    check_divw(INT32_MIN, -1, (int64_t)INT32_MIN);
    check_divw(INT32_MIN + 1, 1, (int64_t)(INT32_MIN + 1));

    check_divuw(10, 3, 3);
    check_divuw(0, 5, 0);
    check_divuw(42, 0, -1);
    check_divuw(0x80000000u, 2, (int64_t)(int32_t)0x40000000u);
    check_divuw(0x80000000u, 1, (int64_t)INT32_MIN);
    check_divuw(UINT32_MAX, 1, -1LL);
    check_divuw(6, 2, 3);

    /* rem/remu */
    check_rem(10, 3, 1);
    check_rem(-10, 3, -1);
    check_rem(10, -3, 1);
    check_rem(-10, -3, -1);
    check_rem(42, 0, 42);
    check_rem(-42, 0, -42);
    check_rem(INT64_MIN, -1, 0);

    check_remu(10, 3, 1);
    check_remu(0, 5, 0);
    check_remu(42, 0, 42);
    check_remu(UINT64_MAX, 0, UINT64_MAX);

    /* remw/remuw */
    check_remw(10, 3, 1);
    check_remw(-10, 3, -1);
    check_remw(42, 0, 42);
    check_remw(INT32_MIN, -1, 0);
    check_remw(INT32_MIN, 0, (int64_t)INT32_MIN);

    check_remuw(10, 3, 1);
    check_remuw(42, 0, 42);
    check_remuw(0x80000000u, 0, (int64_t)INT32_MIN);
    check_remuw(UINT32_MAX, 0, -1LL);
    check_remuw(0x80000000u, UINT32_MAX, (int64_t)INT32_MIN);

    return failed;
}

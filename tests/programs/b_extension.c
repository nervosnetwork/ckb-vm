// https://github.com/riscv/riscv-bitmanip/tree/master/tests
#include "rvintrin.h"
#include "test_zbb.c"
#include "test_zbs.c"
#include "test_zbp.c"
#include "test_zbe.c"
#include "test_zbc.c"
#include "test_zbr.c"
#include "test_zbm.c"
#include "test_zbt.c"
#include "test_zbf.c"
#include "test_stdc.c"
uint64_t args[128] = {
  0x0000000000000000LL,
  0x0000000000000001LL,
  0xffffffffffffffffLL,
  0x8000000000000000LL,
  0x0000000000000004LL,
  0x0000000000000040LL,
  0x0000000000000080LL,
  0x0000000000002000LL,
  0x0000000000010000LL,
  0x0000000000400000LL,
  0x0000000001000000LL,
  0x0000000100000000LL,
  0x0000008000000000LL,
  0x0000020000000000LL,
  0x0000800000000000LL,
  0x0010000000000000LL,
  0x0100000000000000LL,
  0x1000000000000000LL,
  0x000000000000000dLL,
  0x0000000000000000LL,
  0x0000000000000067LL,
  0x2e00000000000000LL,
  0x000000000000015bLL,
  0x9420000000000000LL,
  0x00000000000075daLL,
  0xe35a000000000000LL,
  0x000000000003ed82LL,
  0x8b4eb00000000000LL,
  0x000000000000714cLL,
  0xfad4dc0000000000LL,
  0x000000000dd2966bLL,
  0x686f332000000000LL,
  0x00000000a865d7d4LL,
  0x6edd225600000000LL,
  0x0000000380f3cf69LL,
  0xaf29109cc0000000LL,
  0x000000a3714b9ad2LL,
  0x7dc2ae94e4000000LL,
  0x00000bea6a6af755LL,
  0xea2177d8d5100000LL,
  0x00004a9e26b7f794LL,
  0x6d159abfb3030000LL,
  0x00020e6dfbb7c441LL,
  0xd251a40a022b9000LL,
  0x00129af7f2440efeLL,
  0xc7dee68fffbaf900LL,
  0x05ada4e53975b451LL,
  0x63eb500cce126b70LL,
  0x314320aa7da5b1efLL,
  0xd27d2fde3497614cLL,
  0xbe55668178139c8eLL,
  0x9480583abdfb5837LL,
  0x9d8dbb3a5bde4347LL,
  0x61fd04828c93ce01LL,
  0xdf9a26c8470349ddLL,
  0xca9d54bd4e78980eLL,
  0xb1db9b0fecbfaabeLL,
  0xe79541e25d0dba6bLL,
  0xff98837fda2a5bdfLL,
  0xc3bd5e2cd52318a8LL,
  0x02ab7bb54e687499LL,
  0xbebf0929f41aa230LL,
  0x58aee9fdc3f41b74LL,
  0x62daff171a9fae42LL,
  0xe5baa16ee5b5419eLL,
  0x16b3a918e4278c9dLL,
  0x4ab9cfc9a41744c4LL,
  0x86ddce906c8cdb4dLL,
  0x867e3492977cb1bbLL,
  0x3d0e482377794618LL,
  0x90e1bc8ba22d3294LL,
  0xf48119b103954df1LL,
  0x79780d4e5b2b3b2aLL,
  0xb36eb1caa58ee7dcLL,
  0xf0fe55be95a18d13LL,
  0x1234769364d9eac9LL,
  0x31a7445bdf8bcb5cLL,
  0x1735808ee4398bcaLL,
  0x8f09996552504a5dLL,
  0x4fcf7212bebfdd89LL,
  0xdfd3a0870f60e072LL,
  0x25474d793f2c7d32LL,
  0xb9e2a99fdb7b2948LL,
  0x0da24e08451a8d1aLL,
  0x44a705073f90be80LL,
  0x7f2e6910bdea3ffdLL,
  0x7fc92593c865b4c2LL,
  0x0f812a265e560f2bLL,
  0xfecee737556609f5LL,
  0x996d1b60923c18a6LL,
  0x2c1fb5204d248917LL,
  0x4cf560811e3465c5LL,
  0xf2a6b292a535dc4eLL,
  0x3b4de2fabe6d6476LL,
  0xa6a669d1baba633eLL,
  0xa73c905bcbc01878LL,
  0x38be984c83ce8648LL,
  0x262a15662b298944LL,
  0xdf09e5c90a990b56LL,
  0xa8519a5b46242cc0LL,
  0x14d93f0c55095499LL,
  0xbad28e0ca5854070LL,
  0x93d7d7a9d87056f0LL,
  0x3b0d936889b10a5dLL,
  0x0ec6680cabb95f09LL,
  0x27429c30e8b6cff7LL,
  0x6465f271027abfa8LL,
  0xd0abd7d3688aa0d7LL,
  0x986a686578456056LL,
  0xc10a152d71cb3f16LL,
  0x4a6c986967d5ace8LL,
  0x37269c228e8e3db1LL,
  0xf5bad73c74be6d8aLL,
  0x68323fe289df33d1LL,
  0xcb9848f06e9659f6LL,
  0x5052886f7169c8c5LL,
  0xb040414dd8c98a14LL,
  0xea59a91078581c00LL,
  0x7c6bcb08155fac38LL,
  0xbd6192029dd91d60LL,
  0x8a4a182923bdf75aLL,
  0x8c91e2fe14041a34LL,
  0xc9d368e6546c1f00LL,
  0xdfd83d690e5f073eLL,
  0x34f2a050c605b6b0LL,
  0xf3fbe985738811ddLL,
  0x2d21e3da342cd6beLL,
  0x31523358d080e093LL
};

int main()
{
  long test_zbb_result = test_zbb(args, 128);
  if (test_zbb_result != 0x90f45e4e) return 1;
  long test_zbs_result = test_zbs(args, 128);
  if (test_zbs_result != 0xe4fa9ed0) return 2;
  long test_zbp_result = test_zbp(args, 128);
  if (test_zbp_result != 0xbf7dc8c4) return 3;
  long test_zbe_result = test_zbe(args, 128);
  if (test_zbe_result != 0xad938c6a) return 4;
  long test_zbc_result = test_zbc(args, 128);
  if (test_zbc_result != 0xcdef75a4) return 5;
  long test_zbr_result = test_zbr(args, 128);
  if (test_zbr_result != 0x31fc780f) return 6;
  long test_zbm_result = test_zbm(args, 128);
  if (test_zbm_result != 0x355a32a3) return 7;
  long test_zbt_result = test_zbt(args, 128);
  if (test_zbt_result != 0xcef7df02) return 8;
  long test_zbf_result = test_zbf(args, 128);
  if (test_zbf_result != 0x3fa35b76) return 9;
  long test_stdc_result = test_stdc(args, 128);
  if (test_stdc_result != 0x5a983d30) return 10;
  return 0;
}

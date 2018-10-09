#include <stdint.h>

int main() {
  int small = 0;
  small += 3;
  if (small != 3) { return 1; }

  small *= 3;
  if (small != 9) { return 2; }

  small = small << 3;
  if (small != 72) { return 3; }

  small = small >> 3;
  if (small != 9) { return 4; }

  small = small & 0xffff;
  if (small != 9) { return 5; }

  small = small | 0x2;
  if (small != 11) { return 6; }

  small /= 2;
  if (small != 5) { return 7; }


  uint64_t big = 7;
  big += 3;
  if (big != 10) { return 8; }

  big *= 17;
  if (big != 170) { return 9; }

  big = big << 3;
  if (big != 1360) { return 10; }

  big = big >> 3;
  if (big != 170) { return 11; }

  big = big & 0xffff;
  if (big != 170) { return 12; }

  big = big | 0x5;
  if (big != 175) { return 13; }

  big /= 2;
  if (big != 87) { return 14; }

  int small2 = 8;
  small2 += 2;
  // 5 + 82 = 87
  small += 82;
  if ((small * small2) != 870) { return 15; }
  if ((small / small2) != 8) { return 16; }
  if ((small % small2) != 7) { return 17; }
  if ((small ^ small2) != 93) { return 18; }
  if ((small | small2) != 95) { return 19; }
  if ((small & small2) != 2) { return 20; }

  unsigned int usmall = 5;
  unsigned int usmall2 = 8;
  usmall += 82;
  usmall2 += 2;
  if ((usmall * usmall2) != 870) { return 21; }
  if ((usmall / usmall2) != 8) { return 22; }
  if ((usmall % usmall2) != 7) { return 23; }
  if ((usmall ^ usmall2) != 93) { return 24; }
  if ((usmall | usmall2) != 95) { return 25; }
  if ((usmall & usmall2) != 2) { return 26; }

  int neg_small = 5;
  int neg_small2 = 8;
  neg_small -= 499;
  neg_small2 -= 13;
  if ((neg_small * neg_small2) != 2470) { return 27; }
  if ((neg_small / neg_small2) != 98) { return 28; }
  if ((neg_small % neg_small2) != -4) { return 29; }
  if ((neg_small ^ neg_small2) != 489) { return 30; }
  if ((neg_small | neg_small2) != -5) { return 31; }
  if ((neg_small & neg_small2) != -494) { return 32; }
  return 0;

  int small4 = 8;
  small4 += 13;
  if ((neg_small * small4) != -10374) { return 33; }
  if ((neg_small / small4) != -23) { return 34; }
  if ((neg_small % small4) != 10) { return 35; }
  if ((neg_small ^ small4) != -505) { return 36; }
  if ((neg_small | small4) != -489) { return 37; }
  if ((neg_small & small4) != -16) { return 38; }
  return 0;
}

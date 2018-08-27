
typedef signed long long int 	int64_t;
typedef unsigned long long int 	uint64_t;

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

  return 0;
}

int main() {
  int a = 0x501;

  int b = a & ~(0x7);

  if (b != 0x500) { return 1; }

  return 0;
}

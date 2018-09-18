int main(int argc, char *argv[]) {
  int (*func)() = argv[0];

  return func();
}

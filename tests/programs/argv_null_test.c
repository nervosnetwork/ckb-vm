/*
 * See https://github.com/nervosnetwork/ckb-vm/issues/98 for more details.
 */
#include <stddef.h>

int main(int argc,  char *argv[]) {
  if (argv[argc] == NULL) {
    return 0;
  }
  return 1;
}

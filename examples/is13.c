#include <string.h>

int is13(char* data) {
    if (strcmp(data, "13") == 0) {
        return 0;
    }
    if (strcmp(data, "0xd") == 0) {
        return 0;
    }
    if (strcmp(data, "0o15") == 0) {
        return 0;
    }
    if (strcmp(data, "0b1101") == 0) {
        return 0;
    }
    return 1;
}

int main(int argc, char* argv[]) {
    if (argc == 1) {
        return 1;
    }
    return is13(argv[1]);
}


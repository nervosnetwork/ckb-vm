int main() {
    int a[256*1024] = {};
    for (int i = 0; i < 256*1024; i++) {
        if (a[i] != 0) {
            return 1;
        }
    }
    return 0;
}

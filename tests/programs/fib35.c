int fib(int n) {
    if (n <= 1) {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

int main() {
    // A calculation problem, it takes about 2 seconds in asm.
    if (fib(35) == 9227465) {
        return 0;
    };
    return 1;
}

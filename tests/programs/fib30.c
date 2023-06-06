int fib(int n) {
    if (n <= 1) {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

int main() {
    // A calculation problem, it takes about 2 seconds in interpreter.
    if (fib(30) == 832040) {
        return 0;
    };
    return 1;
}

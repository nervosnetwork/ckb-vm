const int n = 2;

int a() {
    return n;
}

int b() {
    return a() + n;
}

int c() {
    return b() + a();
}

int main() {
    return c() + b() - 10;
}

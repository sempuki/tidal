int main() { int a[5] = {1, 2, 3, 4, 5}; unsigned i = 5; int s = 0; do { --i; s = s * 10 + a[i]; } while (i != 0); return s == 54321 ? 0 : 1; }

int take(const int&) { return 1; } int take(int&&) { return 2; }
int scale(int x, int k = 10) { return x * k; }
int main() { int a = 1; return take(a) == 1 && take(2) == 2 && scale(3) == 30 && scale(3, 2) == 6 ? 0 : 1; }

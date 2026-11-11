// A loop reads one element past the end of an array.
int sum(const int* a, int n) { int s = 0; for (int i = 0; i <= n; ++i) s += a[i]; return s; }
int main() { int a[4] = {1, 2, 3, 4}; return sum(a, 4) == 10 ? 0 : 1; }

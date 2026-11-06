int sum(const int* a, int n) { int s = 0; for (int i = 0; i < n; ++i) s += a[i]; return s; }
int main() { int a[3] = {2147483000, 1000, 5}; return sum(a, 3) > 0 ? 0 : 1; }

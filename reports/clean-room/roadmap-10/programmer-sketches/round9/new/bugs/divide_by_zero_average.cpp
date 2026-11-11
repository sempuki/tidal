int average(const int* a, int n) { int s = 0; for (int i = 0; i < n; ++i) s += a[i]; return s / n; }
int main() { int a[1] = {4}; return average(a, 0); }

// Planted bug: a pointer to an element is kept while the buffer grows and is freed.
struct Vec { int* d = new int[1]; int n = 0, cap = 1;
  void push(int v) { if (n == cap) { int* nd = new int[cap * 2]; for (int i = 0; i < n; ++i) nd[i] = d[i]; delete[] d; d = nd; cap *= 2; } d[n++] = v; }
  ~Vec() { delete[] d; } };
int main() { Vec v; v.push(1); int* first = &v.d[0]; v.push(2); return *first - 1; }

template <bool Const> struct Iter {
  const int* p;
  int k;
  Iter(const int* q, int n) : p(q), k(n) {}
};
Iter<true> first(const int* a) { return {a, 0}; }
int main() { int a[1] = {0}; return *first(a).p + first(a).k; }

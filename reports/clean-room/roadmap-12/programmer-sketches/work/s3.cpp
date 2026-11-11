template <bool Const> struct Iter {
  const int* p;
  int k;
  Iter(const int* q, int n) : p(q), k(n) {}
  template <bool C = Const>
    requires C
  Iter(const Iter<false>& o) : p(o.p), k(o.k) {}
};
Iter<true> first(const int* a) { return {a, 0}; }
int main() { int a[1] = {0}; return *first(a).p + first(a).k; }

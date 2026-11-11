template <bool Const> struct Iter {
  const int* p;
  Iter(const int* q) : p(q) {}
  template <bool C = Const>
    requires C
  Iter(const Iter<false>& o) : p(o.p) {}
};
Iter<true> first(const int* a) { return {a}; }
int main() { int a[1] = {0}; return *first(a).p; }

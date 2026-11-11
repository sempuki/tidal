template <bool Const> struct Iter {
  const int* p;
  Iter(const int* q) : p(q) {}
  template <bool C = Const>
  Iter(const Iter<false>& o) : p(o.p) {}
};
int main() { int a[1] = {0}; Iter<true> i{a}; return *i.p; }

struct Iter {
  const int* p;
  Iter(const int* q) : p(q) {}
  template <bool C = true>
    requires C
  Iter(const Iter& o, int) : p(o.p) {}
};
int main() { int a[1] = {0}; Iter i{a}; return *i.p; }

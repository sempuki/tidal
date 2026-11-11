template <bool Const> struct Iter {
  int* p;
  explicit Iter(int* q) : p(q) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : p(o.p) {}
};
int take(Iter<true> pos) { return *pos.p; }
int main() {
  int n = 0;
  Iter<false> m(&n);
  Iter<false> m2 = m;
  return take(m2);
}

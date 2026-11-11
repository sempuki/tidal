template <bool Const> struct Iter {
  int* p;
  explicit Iter(int* q) : p(q) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : p(o.p) {}
};
Iter<false> make(Iter<true> pos) { return Iter<false>(pos.p); }
int main() {
  int n = 0;
  Iter<false> m(&n);
  return *make(m).p;
}

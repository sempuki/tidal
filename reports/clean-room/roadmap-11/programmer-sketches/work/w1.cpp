// An iterator that converts to its const form through a constrained
// constructor template, beside a constructor from a pointer. The front end
// finds no conversion for the argument (OVL-35).
template <bool Const> struct Iter {
  int* n_ = nullptr;
  explicit Iter(int* n) : n_(n) {}
  template <bool C = Const> requires C
  Iter(const Iter<false>& o) : n_(o.n_) {}
};
int take(Iter<true> it) { return *it.n_; }
int main() {
  int n = 0;
  Iter<false> m(&n);
  return take(m);
}

// tidal-rules: RFOR-2
// tidal-expect: run
// tidal-note: P3-9. R has member begin and end from its base Base<R>, a specialization over R itself. RFOR-2 finds the members by lookup in R, which searches the base. GCC, Clang, and EDG accept, and the loop sums 0, 1, and 2.
template <class S> struct It {
  int i;
  It& operator++() { ++i; return *this; }
  int operator*() const { return i; }
  bool operator!=(It o) const { return i != o.i; }
};
template <class D> struct Base {
  It<D> begin() { return It<D>{0}; }
  It<D> end() { return It<D>{3}; }
};
struct R : Base<R> {};
int main() { R r; int s = 0; for (int x : r) s += x; return s - 3; }

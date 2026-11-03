// Returning a named guard with deleted copy and no move: CPY-P4 says a
// local in return is treated as an rvalue; with no move, is it rejected?
template <class F> struct Defer {
  F f; explicit Defer(F g) : f(g) {}
  Defer(const Defer&) = delete;
  ~Defer() { f(); }
};
template <class F> Defer<F> make(F f) { Defer<F> d(f); return d; }
int main() { int n = 0; { auto g = make([&] { ++n; }); } return n == 1 ? 0 : 1; }

// A lambda that captures a function parameter pack by copy ([xs...]) and
// folds over it. An init-capture pack ([... ys = xs]) runs.
template <class... X> auto keep(X... xs) {
  return [xs...](int r) { return (xs + ... + r); };
}
int main() { auto p = keep(1, 2); return p(3) == 6 ? 0 : 1; }

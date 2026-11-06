// A generic lambda that captures a pack by copy ([xs...]) and folds over it.
template <class... X> auto keep(X... xs) {
  return [xs...](auto r) { return (xs + ... + r); };
}
int main() { auto p = keep(1, 2); return p(3) == 6 ? 0 : 1; }

// tidal-rules: PACK-17
// tidal-expect: accept
// tidal-note: x... captures each element by copy, and &x... by reference. With the default =, the pack capture is by reference, and with &, by copy.
template <class... T> constexpr int by_copy(T... x) { auto f = [x...] { return (x + ... + 0); }; return f(); }
static_assert(by_copy(1, 2, 3) == 6);
template <class... T> constexpr int by_reference(T... x) {
  auto f = [&x...] { ((x = 1), ...); };
  f();
  return (x + ... + 0);
}
static_assert(by_reference(5, 5, 5) == 3);
template <class... T> constexpr int with_defaults(T... x) {
  auto g = [=, &x...] { return sizeof...(x); };
  auto h = [&, x...] { return sizeof...(x); };
  return g() + h();
}
static_assert(with_defaults(1, 2) == 4);

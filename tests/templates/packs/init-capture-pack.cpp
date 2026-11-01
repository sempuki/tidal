// tidal-rules: PACK-18
// tidal-expect: accept
// tidal-note: ...y = e makes one member per element of e, and &...y = x one reference per element. The name y is a pack in the body.
template <class... T> constexpr int doubled(T... x) { auto f = [...y = x * 2] { return (y + ... + 0); }; return f(); }
static_assert(doubled(1, 2) == 6);
template <class... T> constexpr int through_reference(T... x) {
  auto f = [&...y = x] { ((y = 4), ...); };
  f();
  return (x + ... + 0);
}
static_assert(through_reference(1, 2) == 8);
template <class... T> constexpr int members(T... x) { auto f = [...y = x] { return sizeof...(y); }; return f(); }
static_assert(members() == 0 && members(1, 2, 3) == 3);

// tidal-rules: PACK-17
// tidal-expect: accept
// tidal-note: The body uses p... with a capture default and no explicit capture. With = each element is copied, so the old values remain after the parameters change. With & each element is a reference.
template <class... T> constexpr int sum(T... v) { return (v + ... + 0); }
template <class... T> constexpr int by_copy(T... p) { auto f = [=] { return sum(p...); }; ((p = 0), ...); return f(); }
template <class... T> constexpr int by_reference(T... p) { auto f = [&] { return sum(p...); }; ((p = 0), ...); return f(); }
static_assert(by_copy(1, 2, 3) == 6);
static_assert(by_reference(1, 2, 3) == 0);

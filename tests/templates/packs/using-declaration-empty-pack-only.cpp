// tidal-rules: PACK-30
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: instantiates to an empty pack
// tidal-note: g calls f, which finds using B::f... at the definition. D<> has no bases, so the pack is empty. GCC and EDG call the namespace f, and Clang rejects. PACK-30 allows both (Q-0274).
constexpr int f(long) { return 2; }
struct A { constexpr int f(int) const { return 1; } };
template <class... B> struct D : B... {
  using B::f...;
  constexpr int g() const { return f(0L); }
};
static_assert(D<>().g() == 2);

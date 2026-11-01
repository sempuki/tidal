// tidal-rules: PACK-30
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: instantiates to an empty pack
// tidal-note: f in g finds using B::f... at the definition. In D<> the pack is empty. GCC and EDG then call the namespace f, and Clang rejects. PACK-30 allows both (Q-0274).
constexpr int f(int) { return 2; }
struct A { constexpr int f(int) const { return 1; } };
template <class... B> struct D : B... {
  using B::f...;
  constexpr int g() const { return f(0); }
};
static_assert(D<A>().g() == 1);
static_assert(D<>().g() == 2);

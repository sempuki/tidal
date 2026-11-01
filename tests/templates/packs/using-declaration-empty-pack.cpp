// tidal-rules: PACK-21
// tidal-expect: accept
// tidal-note: Merge<> has no bases, so using Bs::f... declares nothing, and the call finds only the f of Merge. Merge<Small> also gets f(int) from Small, which matches better.
struct Small { constexpr int f(int) const { return 1; } };
template <class... Bs> struct Merge : Bs... {
  using Bs::f...;
  constexpr int f(double) const { return 0; }
};
static_assert(Merge<>{}.f(1) == 0);
static_assert(Merge<Small>{}.f(1) == 1);

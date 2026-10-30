// tidal-rules: NS-3
// tidal-expect: accept
namespace A {
  inline namespace V { constexpr int x = 1; }
  namespace V { constexpr int y = 2; }
}
static_assert(A::x == 1 && A::y == 2);

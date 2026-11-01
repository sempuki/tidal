// tidal-rules: OVL-6
// tidal-expect: accept
// tidal-note: A surrogate from a conversion function qualified && serves a prvalue object. One qualified & serves an lvalue object.
using Fn = int (*)(int);
constexpr int twice(int x) { return 2 * x; }
struct Temp { constexpr operator Fn() && { return twice; } };
struct Held { constexpr operator Fn() & { return twice; } };
static_assert(Temp{}(3) == 6);
constexpr int probe() {
  Held h;
  return h(4);
}
static_assert(probe() == 8);

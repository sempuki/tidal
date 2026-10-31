// tidal-rules: INIT-9
// tidal-expect: accept
struct Inner { int v = 7; };
struct Outer {
  Inner inner;
  int x;
  constexpr Outer() : x(1) {}
};
constexpr int read() {
  Outer o;
  return o.inner.v + o.x;
}
static_assert(read() == 8);

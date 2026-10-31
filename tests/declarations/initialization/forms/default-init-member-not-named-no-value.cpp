// tidal-rules: INIT-9 OBJ-2
// tidal-expect: reject
// tidal-error-clang: uninitialized
// tidal-error-gcc: uninitialized
// tidal-error-edg: uninitialized
struct Outer {
  int y;
  int x;
  constexpr Outer() : x(1) {}
};
constexpr int read() {
  Outer o;
  return o.y;
}
static_assert(read() == 0);

// tidal-rules: INIT-9 OBJ-2
// tidal-expect: reject
// tidal-error-clang: read of uninitialized object|uninitialized
// tidal-error-gcc: uninitialized
// tidal-error-edg: uninitialized
struct Settings {
  int width = 80;
  int height;
};
constexpr int read() {
  Settings s;
  return s.height;
}
static_assert(read() == 0);

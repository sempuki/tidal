// tidal-rules: CX-3
// tidal-expect: reject
constexpr int f() {
  static int calls = 0;
  return ++calls;
}
static_assert(f() == 1);

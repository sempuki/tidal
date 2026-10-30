// tidal-rules: CX-3
// tidal-expect: accept
constexpr int f(bool b) {
  if (b) return 1;
  static int calls = 0;
  return ++calls;
}
static_assert(f(true) == 1);

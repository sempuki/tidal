// tidal-rules: CX-6
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: calls. defined .static. in .constexpr. context
// tidal-error-edg: must have a constant value
constexpr int f() {
  static int calls = 0;
  return ++calls;
}
static_assert(f() == 1);

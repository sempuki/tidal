// tidal-rules: FN-3
// tidal-expect: reject
// tidal-error-gcc: flows off the end of the function
// tidal-error-clang: control reached end of constexpr function
// tidal-error-edg: missing return value
constexpr int f(bool b) {
  if (b) return 1;
}
static_assert(f(false) == 0);

// tidal-rules: CMP-14
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: chained comparison
// tidal-error-gcc: do not have their mathematical meaning
// tidal-note: Q-0029. Clang 18 gives no -Wparentheses warning for a chained comparison of variables.
// tidal-flags: -Werror -Wparentheses
bool f(int a, int b, int c) {
  return a < b < c;
}

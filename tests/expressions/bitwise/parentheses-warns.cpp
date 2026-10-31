// tidal-rules: BIT-6
// tidal-expect: reject
// tidal-error-clang: lower precedence than ==
// tidal-error-gcc: suggest parentheses around comparison
// tidal-flags: -Werror -Wparentheses
bool f(int a, int b, int c) {
  return a & b == c;
}

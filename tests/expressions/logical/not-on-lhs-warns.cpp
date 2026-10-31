// tidal-rules: LOG-9
// tidal-expect: reject
// tidal-error-clang: logical not is only applied to the left hand side
// tidal-error-gcc: logical not is only applied to the left hand side
// tidal-flags: -Werror -Wlogical-not-parentheses
bool f(int a, int b) {
  return !a == b;
}

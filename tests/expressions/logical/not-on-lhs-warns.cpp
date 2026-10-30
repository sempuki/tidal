// tidal-rules: LOG-9
// tidal-expect: reject
// tidal-flags: -Werror -Wlogical-not-parentheses
bool f(int a, int b) {
  return !a == b;
}

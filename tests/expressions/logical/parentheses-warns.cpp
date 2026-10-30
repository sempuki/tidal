// tidal-rules: LOG-8
// tidal-expect: reject
// tidal-flags: -Werror -Wparentheses
bool f(bool a, bool b, bool c) {
  return a || b && c;
}

// tidal-rules: LOG-8
// tidal-expect: accept
// tidal-flags: -Werror -Wparentheses
bool f(bool a, bool b, bool c) {
  return a || (b && c);
}

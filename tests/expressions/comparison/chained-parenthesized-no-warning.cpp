// tidal-rules: CMP-14
// tidal-expect: accept
// tidal-flags: -Werror -Wparentheses
bool first(int a, int b, int c) {
  return (a < b) < c;
}
bool second(int a, int b, int c) {
  return a < (b < c);
}

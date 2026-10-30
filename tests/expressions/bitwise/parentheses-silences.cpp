// tidal-rules: BIT-6
// tidal-expect: accept
// tidal-flags: -Werror -Wparentheses
bool f(int a, int b, int c) {
  return (a & b) == c;
}

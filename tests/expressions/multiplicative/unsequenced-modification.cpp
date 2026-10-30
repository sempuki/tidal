// tidal-rules: MUL-14
// tidal-expect: reject
// tidal-flags: -Wall -Werror
int f(int i) {
  return i++ * i;
}

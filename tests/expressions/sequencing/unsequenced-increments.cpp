// tidal-rules: SEQ-4
// tidal-expect: reject
// tidal-flags: -Wall -Werror
int f(int i) {
  return i++ + i++;
}

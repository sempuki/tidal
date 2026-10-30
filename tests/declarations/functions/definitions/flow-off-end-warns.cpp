// tidal-rules: FN-7
// tidal-expect: reject
// tidal-flags: -Werror -Wreturn-type
int f(bool b) {
  if (b) return 1;
}

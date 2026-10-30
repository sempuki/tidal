// tidal-rules: FN-3
// tidal-expect: reject
// tidal-flags: -Werror -Wreturn-type
int f(bool b) {
  if (b) return 1;
}

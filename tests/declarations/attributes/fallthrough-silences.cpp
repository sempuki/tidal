// tidal-rules: ATTR-7
// tidal-expect: accept
// tidal-flags: -Werror -Wimplicit-fallthrough
int f(int k) {
  int r = 0;
  switch (k) {
    case 0:
      r += 1;
      [[fallthrough]];
    case 1:
      r += 2;
      break;
  }
  return r;
}

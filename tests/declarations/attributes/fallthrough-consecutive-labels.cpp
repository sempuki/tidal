// tidal-rules: ATTR-7
// tidal-expect: accept
// tidal-flags: -Werror -Wimplicit-fallthrough
int pick(int k) {
  int r = 0;
  switch (k) {
    case 0:
    case 1:
      r = 1;
      break;
    default:
      r = 2;
  }
  return r;
}

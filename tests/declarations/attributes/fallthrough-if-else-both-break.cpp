// tidal-rules: ATTR-7
// tidal-expect: accept
// tidal-flags: -Werror -Wimplicit-fallthrough
int pick(int k, bool wide) {
  int r = 0;
  switch (k) {
    case 0:
      if (wide) {
        r = 1;
        break;
      } else {
        r = 2;
        break;
      }
    case 1:
      r = 3;
      break;
  }
  return r;
}

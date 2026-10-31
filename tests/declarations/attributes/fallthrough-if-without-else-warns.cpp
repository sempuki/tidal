// tidal-rules: ATTR-7
// tidal-expect: reject
// tidal-error-clang: unannotated fall-through between switch labels
// tidal-error-gcc: this statement may fall through
// tidal-flags: -Werror -Wimplicit-fallthrough
int pick(int k, bool wide) {
  int r = 0;
  switch (k) {
    case 0:
      if (wide) {
        r = 1;
        break;
      }
    case 1:
      r = 3;
      break;
  }
  return r;
}

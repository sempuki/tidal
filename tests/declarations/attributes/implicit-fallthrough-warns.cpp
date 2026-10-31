// tidal-rules: ATTR-7
// tidal-expect: reject
// tidal-error-clang: unannotated fall-through between switch labels
// tidal-error-gcc: this statement may fall through
// tidal-flags: -Werror -Wimplicit-fallthrough
int f(int k) {
  int r = 0;
  switch (k) {
    case 0:
      r += 1;
    case 1:
      r += 2;
      break;
  }
  return r;
}

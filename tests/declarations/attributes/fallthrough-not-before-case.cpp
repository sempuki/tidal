// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: fallthrough annotation does not directly precede switch label
// tidal-error-gcc: fallthrough. not preceding a case label
// tidal-error-edg: fallthrough statement must precede switch case label
int f(int k) {
  switch (k) {
    case 0:
      [[fallthrough]];
      k = 2;
    case 1:
      return k;
  }
  return 0;
}

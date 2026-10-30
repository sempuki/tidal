// tidal-rules: ATTR-2
// tidal-expect: reject
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

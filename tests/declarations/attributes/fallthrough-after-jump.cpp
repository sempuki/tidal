// tidal-rules: ATTR-7
// tidal-expect: accept
// tidal-flags: -Werror -Wimplicit-fallthrough
int pick(int k) {
  int r = 0;
  for (int i = 0; i < 2; ++i) {
    switch (k) {
      case 0:
        ++r;
        continue;
      case 1:
        ++r;
        goto done;
      case 2:
        throw r;
      case 3:
        return r;
      default:
        r = 9;
    }
  }
done:
  return r;
}

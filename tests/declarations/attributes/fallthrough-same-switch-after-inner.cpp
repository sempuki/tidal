// tidal-rules: ATTR-2
// tidal-expect: accept
// tidal-note: The fallthrough is in the outer switch, after the inner switch, and the next statement has a case label of the outer switch.
int f(int a, int b) {
  int r = 0;
  switch (a) {
  case 0:
    switch (b) {
    case 1:
      r = 1;
    }
    [[fallthrough]];
  case 2:
    r += 2;
  }
  return r;
}

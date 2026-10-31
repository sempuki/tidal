// tidal-rules: ATTR-2
// tidal-expect: accept
void act();
void step(int state, bool more) {
  switch (state) {
  case 0:
    if (more) {
      act();
      [[fallthrough]];
    }
  case 1:
    act();
  }
}

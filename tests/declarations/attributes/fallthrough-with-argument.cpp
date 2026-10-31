// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: cannot have an argument list
// tidal-error-gcc: does not take any arguments
// tidal-error-edg: does not take arguments
void act();
void step(int state) {
  switch (state) {
  case 0:
    act();
    [[fallthrough(1)]];
  case 1:
    act();
  }
}

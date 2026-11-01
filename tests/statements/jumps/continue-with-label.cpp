// tidal-rules: JMP-17
// tidal-expect: reject
// tidal-error-clang: named .continue.|expected .;. after .?continue
// tidal-error-gcc: expected .;. before .outer.
// tidal-error-edg: expected a .;.
int f() {
  int hits = 0;
outer:
  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 3; ++j) {
      if (i * j == 2) continue outer;
      ++hits;
    }
  }
  return hits;
}

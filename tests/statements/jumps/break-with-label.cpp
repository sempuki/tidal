// tidal-rules: JMP-17
// tidal-expect: reject
// tidal-error-clang: named .break.|expected .;. after .?break
// tidal-error-gcc: expected .;. before .outer.
// tidal-error-edg: expected a .;.
int f() {
  int hits = 0;
outer:
  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 3; ++j) {
      if (i * j == 2) break outer;
      ++hits;
    }
  }
  return hits;
}

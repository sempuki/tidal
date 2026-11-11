// tidal-rules: FN-7
// tidal-expect: accept
// tidal-flags: -Werror -Wreturn-type
// tidal-note: X24-4. Each break ends only the inner switch or for, so no break leaves the loop whose condition is a constant true, and control cannot reach the end of the function.
extern int next_code();
int f() {
  for (;;) {
    switch (next_code()) {
    case 0:
      break;
    default:
      return 1;
    }
    for (int i = 0; i < 3; ++i) {
      if (i == next_code()) break;
    }
  }
}

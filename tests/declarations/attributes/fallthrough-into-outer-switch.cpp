// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: not preceding a case label or default label
// tidal-error-edg: fallthrough statement must precede switch case label
// tidal-note: Q-0390. The fallthrough ends the inner switch, and the next statement that can run has a case label of the outer switch. GCC and EDG reject it, as the gap review of the text phase 8 wrote reads the C++23 text. GCC reports the error only when it compiles the code, not with -fsyntax-only. Clang accepts it.
int f(int a, int b) {
  int r = 0;
  switch (a) {
  case 0:
    switch (b) {
    case 1:
      r = 1;
      [[fallthrough]];
    }
  case 2:
    r += 2;
  }
  return r;
}

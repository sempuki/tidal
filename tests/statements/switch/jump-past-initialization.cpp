// tidal-rules: SW-5
// tidal-expect: reject
// tidal-error-clang: cannot jump from switch statement to this case label
// tidal-error-gcc: jump to case label
// tidal-error-edg: transfer of control bypasses initialization
void f(int v) {
  switch (v) {
    case 1:
      int x = 1;
    case 2:
      break;
  }
}

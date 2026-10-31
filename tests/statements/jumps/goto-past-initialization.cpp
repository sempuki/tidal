// tidal-rules: JMP-6
// tidal-expect: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-gcc: jump to label
// tidal-error-edg: transfer of control bypasses initialization
void f() {
  goto skip;
  int x = 1;
skip:
  (void)x;
}

// tidal-rules: JMP-6
// tidal-expect: reject
void f() {
  goto skip;
  int x = 1;
skip:
  (void)x;
}

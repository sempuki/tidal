// tidal-rules: JMP-6
// tidal-expect: accept
int f() {
  goto skip;
  int x;
skip:
  x = 2;
  return x;
}

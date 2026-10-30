// tidal-rules: JMP-6
// tidal-expect: accept
int f() {
  goto skip;
  static int s = 1;
skip:
  return s;
}

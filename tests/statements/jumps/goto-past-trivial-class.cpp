// tidal-rules: JMP-6
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct Plain { int v; };
int f() {
  goto skip;
  Plain p;
skip:
  return 0;
}

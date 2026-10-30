// tidal-rules: IF-7
// tidal-expect: accept
void f() {
  if (static int n = 1; n) {}
  if (struct S { int v; } s{1}; s.v) {}
  if (int g(); true) {}
}

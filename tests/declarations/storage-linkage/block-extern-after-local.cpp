// tidal-rules: STO-7
// tidal-expect: reject
void f() {
  int x = 0;
  extern int x;
}

// tidal-rules: LAM-10
// tidal-expect: reject
void f() {
  int n = 0;
  int (*p)() = [n] { return n; };
}

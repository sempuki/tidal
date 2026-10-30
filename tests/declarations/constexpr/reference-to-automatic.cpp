// tidal-rules: CX-2
// tidal-expect: reject
void f() {
  int local = 1;
  constexpr int& r = local;
}

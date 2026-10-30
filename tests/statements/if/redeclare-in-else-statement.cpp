// tidal-rules: IF-10
// tidal-expect: reject
void f() {
  if (auto x = 1) {} else int x = 2;
}

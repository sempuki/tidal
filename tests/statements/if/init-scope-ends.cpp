// tidal-rules: IF-9
// tidal-expect: reject
int f() {
  if (int a = 1; a) {}
  return a;
}

// tidal-rules: CLS-12
// tidal-expect: reject
int f() {
  int local = 1;
  struct S { int get() { return local; } };
  return S{}.get();
}

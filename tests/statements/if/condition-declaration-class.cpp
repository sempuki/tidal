// tidal-rules: IF-6
// tidal-expect: reject
void f() {
  if (struct S { int v; explicit operator bool() const { return v; } } s{1}) {}
}

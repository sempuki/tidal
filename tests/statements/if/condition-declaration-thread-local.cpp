// tidal-rules: IF-6
// tidal-expect: reject
void f() {
  if (thread_local int x = 1) {}
}

// tidal-rules: LOOP-1
// tidal-expect: reject
struct S {};
void f() {
  while (S{}) {}
}

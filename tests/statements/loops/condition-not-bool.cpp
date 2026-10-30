// tidal-rules: LOOP-5
// tidal-expect: reject
struct S {};
void f() {
  while (S{}) {}
}

// tidal-rules: IF-4
// tidal-expect: reject
struct Empty {};
void f() {
  if (Empty{}) {}
}

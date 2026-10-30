// tidal-rules: UN-6
// tidal-expect: reject
struct Incomplete;
void f(Incomplete* p) {
  auto copy = *p;
}

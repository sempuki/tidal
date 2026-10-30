// tidal-rules: REF-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct B { int v : 4; };
int main() {
  B b{3};
  const int& r = b.v;
  b.v = 5;
  return r == 3 ? 0 : 1;
}

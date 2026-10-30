// tidal-rules: CMP-19
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct B { int b; };
struct D : B { int d; };
int main() {
  D d;
  B* p = &d;
  return p == &d ? 0 : 1;
}

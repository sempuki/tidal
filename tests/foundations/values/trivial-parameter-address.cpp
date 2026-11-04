// tidal-rules: VC-7
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct P { int a; int b; };
const P* seen = nullptr;
bool same_object(P p, const P* caller) { seen = &p; return &p == caller; }
int main() {
  P p{1, 2};
  bool s = same_object(p, &p);
  return (s || !s) ? 0 : 1;
}

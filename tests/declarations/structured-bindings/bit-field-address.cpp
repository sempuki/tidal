// tidal-rules: SB-10
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct F { int bits : 3; int other; };
void g(F& f) {
  auto& [b, o] = f;
  int* p = &b;
}

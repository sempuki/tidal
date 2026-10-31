// tidal-rules: SB-16
// tidal-expect: reject
// tidal-error-clang: address of bit-field requested
// tidal-error-gcc: take address of bit-field
// tidal-error-edg: address of a bit field is not allowed
// tidal-note: Gap review 2026-10-30.
struct F { int bits : 3; int other; };
void g(F& f) {
  auto& [b, o] = f;
  int* p = &b;
}

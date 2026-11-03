// tidal-rules: SB-16
// tidal-expect: reject
// tidal-error-clang: address of bit-field requested
// tidal-error-gcc: take address of bit-field
// tidal-error-edg: address of a bit field is not allowed
// tidal-note: The hidden variable is a copy, and its member is still a bit-field.
struct F { unsigned bits : 4; int other; };
void g(F f) {
  auto [b, o] = f;
  auto p = &b;
}

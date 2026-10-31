// tidal-rules: IF-4
// tidal-expect: reject
// tidal-error-clang: not contextually convertible to .bool
// tidal-error-gcc: could not convert .* to .bool
// tidal-error-edg: must have bool type
struct Empty {};
void f() {
  if (Empty{}) {}
}

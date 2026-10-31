// tidal-rules: LOOP-5
// tidal-expect: reject
// tidal-error-clang: not contextually convertible to .bool
// tidal-error-gcc: could not convert .* to .bool
// tidal-error-edg: must have bool type
struct S {};
void f() {
  while (S{}) {}
}

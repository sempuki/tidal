// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-error-clang: explicit object member function cannot have .const. qualifier
// tidal-error-gcc: explicit object member function .* cannot have cv-qualifier
// tidal-error-edg: type qualifier is not allowed on a static member function
// tidal-note: Gap review 2026-10-30.
struct S {
  int get(this S self) const;
};

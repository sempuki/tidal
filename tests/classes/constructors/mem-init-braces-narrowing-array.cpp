// tidal-rules: CTOR-21
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion
// tidal-error-edg: narrowing conversion
// tidal-note: The member is an array, and the braces give one of its elements a double.
struct S {
  int parts[2];
  S(double d) : parts{1, d} {}
};

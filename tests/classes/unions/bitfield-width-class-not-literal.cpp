// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: non-literal type|non-integral type
// tidal-error-edg: must have a constant value
// tidal-note: W has a user-provided destructor, so it is not a literal type, and a temporary of W cannot appear in a constant. GCC rejects every width of class type (Q-0314).
struct W {
  ~W() {}
  constexpr operator int() const { return 3; }
};
struct S { int b : W(); };

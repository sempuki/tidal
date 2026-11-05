// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: non-integral type
// tidal-error-edg: must have a constant value
// tidal-note: The conversion function of W is not constexpr, so the width is not a constant. GCC rejects every width of class type (Q-0314).
struct W { operator int() const { return 3; } };
struct S { int b : W(); };

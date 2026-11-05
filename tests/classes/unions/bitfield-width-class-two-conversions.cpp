// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion
// tidal-error-gcc: non-integral type
// tidal-error-edg: more than one conversion function
// tidal-note: W has two constexpr conversion functions to integer types, so the conversion of the width is ambiguous. GCC rejects every width of class type (Q-0314).
struct W {
  constexpr operator int() const { return 3; }
  constexpr operator long() const { return 3; }
};
struct S { int b : W(); };

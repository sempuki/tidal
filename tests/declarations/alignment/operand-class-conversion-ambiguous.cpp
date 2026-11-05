// tidal-rules: ALN-8
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion
// tidal-error-gcc: non-integral type
// tidal-error-edg: more than one
// tidal-note: Eight converts to int and to long by two constexpr conversion functions, so the conversion to an integer type is ambiguous. GCC rejects every operand of class type (Q-0314), so its rejection has another reason.
struct Eight {
  constexpr operator int() const { return 8; }
  constexpr operator long() const { return 8; }
};
constexpr Eight eight{};
struct alignas(eight) Block { int v; };

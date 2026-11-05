// tidal-rules: ALN-8
// tidal-expect: reject
// tidal-error-clang: requires integer constant
// tidal-error-gcc: non-integral type
// tidal-error-edg: expression must
// tidal-note: The conversion function of Eight is not constexpr, so the converted operand is not a constant. GCC rejects every operand of class type (Q-0314), so its rejection has another reason.
struct Eight { operator int() const { return 8; } };
constexpr Eight eight{};
struct alignas(eight) Block { int v; };

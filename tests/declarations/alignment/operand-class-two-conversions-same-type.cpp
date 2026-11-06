// tidal-rules: ALN-8
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: non-integral type
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md). Q-0314. W has two conversion functions to int. Both give one target type, and the operand uses the constexpr one that the W prvalue of make() selects. Clang and EDG accept. GCC rejects every operand of class type.
struct W {
  constexpr operator int() { return 16; }
  operator int() const { return 2; }
};
constexpr W make() { return {}; }
struct alignas(make()) S { char c; };
static_assert(alignof(S) == 16);

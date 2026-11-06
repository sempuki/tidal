// tidal-rules: BF-1
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: non-integral type
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md). Q-0314. W has two conversion functions to int, a constexpr one for a non-const object and one for a const object. Both give one target type, and the width uses the one that a W prvalue selects, which is constexpr. Clang and EDG accept. GCC rejects every width of class type.
struct W {
  constexpr operator int() { return 4; }
  operator int() const { return 9; }
};
struct S { unsigned f : W{}; };
constexpr S s{15};
static_assert(s.f == 15);

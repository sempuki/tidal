// tidal-rules: CMP-8
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31. Constant evaluation gives a result for this comparison, although the order is unspecified.
struct Empty {};
struct S {
  [[no_unique_address]] Empty tag;
  int value;
};
constexpr S s{};
constexpr bool lt = static_cast<const void*>(&s.tag) < static_cast<const void*>(&s.value);

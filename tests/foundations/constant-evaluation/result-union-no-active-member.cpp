// tidal-rules: CE-21
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: incompletely initialized variable
// tidal-note: Q-0117. Clang and EDG accept a constant union with no active member; GCC rejects it.
union Slot {
  int number;
  float real;
  constexpr Slot() {}
};
constexpr Slot slot;

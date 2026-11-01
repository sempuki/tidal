// tidal-rules: TSP-14 TSP-11
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: does not specialize any template argument
// tidal-note: The partial specialization of the variable template has the parameter auto* P, so only a pointer argument matches it. Clang 18 takes the argument list for the primary template's (Q-0295).
template <auto V> constexpr int kind = 0;
template <auto* P> constexpr int kind<P> = 1;
int n = 0;
static_assert(kind<1> == 0 && kind<&n> == 1);

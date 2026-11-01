// tidal-rules: TSP-14 TSP-11
// tidal-expect: accept
// tidal-note: The partial specialization's parameter P is declared auto*. Only an argument whose type has that form matches it. The argument list Kind<P> differs from the primary's, because auto* is narrower than auto.
template <auto V> struct Kind { static constexpr int value = 0; };
template <auto* P> struct Kind<P> { static constexpr int value = 1; };
int n = 0;
long m = 0;
static_assert(Kind<1>::value == 0 && Kind<'c'>::value == 0 && Kind<nullptr>::value == 0);
static_assert(Kind<&n>::value == 1 && Kind<&m>::value == 1);

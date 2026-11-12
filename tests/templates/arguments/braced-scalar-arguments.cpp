// tidal-rules: TARG-16
// tidal-expect: accept
// tidal-note: RV11-R-7. A braced list with one value, or with none, is the argument for a constant parameter of scalar type. Empty braces give zero and a null pointer. Every compiler accepts each argument.
template <int N> struct Count { static constexpr int value = N; };
template <const int* P> struct Where { static constexpr bool none = P == nullptr; };
static_assert(Count<{3}>::value == 3);
static_assert(Count<{}>::value == 0);
static_assert(Where<{nullptr}>::none);
static_assert(Where<{}>::none);

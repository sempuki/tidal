// tidal-rules: TSP-14 DED-12
// tidal-expect: accept
// tidal-expect-clang-host: reject
// tidal-expect-clang-22: reject
// tidal-error-clang: static assertion failed
// tidal-note: The parameter is declared const auto*. The type of &c is const int*, which has that form. The type of &n is int*, which does not, so Kind<&n> uses the primary template. Clang 22 matches int* to const auto* by a qualification conversion, and Clang 18 does not (Q-0295).
template <auto V> struct Kind { static constexpr int value = 0; };
template <const auto* P> struct Kind<P> { static constexpr int value = 1; };
int n = 0;
const int c = 0;
static_assert(Kind<&c>::value == 1);
static_assert(Kind<&n>::value == 0);

// tidal-rules: TSP-14
// tidal-expect: accept
// tidal-note: A constant argument in the pattern matches an equal value.
template <class T, int N> struct Array { static constexpr int kind = 0; };
template <class T> struct Array<T, 0> { static constexpr int kind = 1; };
template <int N> struct Array<char, N> { static constexpr int kind = 2; };
static_assert(Array<int, 3>::kind == 0);
static_assert(Array<int, 1 - 1>::kind == 1);
static_assert(Array<char, 3>::kind == 2);

// tidal-rules: TSP-10
// tidal-expect: accept
// tidal-note: The partial specializations of the variable template fix the second argument, and match an array type with its bound.
template <class T, int N> constexpr int code = N;
template <class T> constexpr int code<T, 0> = -1;
template <class T, int N> constexpr int code<T[N], N> = 100 + N;
static_assert(code<int, 3> == 3);
static_assert(code<int, 0> == -1);
static_assert(code<char[4], 4> == 104);

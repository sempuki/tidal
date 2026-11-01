// tidal-rules: PACK-13
// tidal-expect: accept
// tidal-note: Each element becomes one argument or one element of the list. The elements of a braced list are evaluated from left to right.
template <class... T> constexpr int count(T... x) { return sizeof...(x); }
template <class... T> constexpr int twice(T... x) { return count(x..., 0, x...); }
static_assert(twice(1, 2) == 5);
template <class... T> constexpr int first(T... x) { int values[] = {x..., 0}; return values[0]; }
static_assert(first(7, 8) == 7 && first() == 0);
template <int... D> constexpr int digits() { int n = 0; int steps[] = {(n = n * 10 + D)...}; (void)steps; return n; }
static_assert(digits<1, 2, 3>() == 123);

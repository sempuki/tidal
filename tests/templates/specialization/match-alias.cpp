// tidal-rules: TSP-14
// tidal-expect: accept
// tidal-note: An alias in the pattern stands for its type, so Pointer<T> matches like T*.
template <class T> using Pointer = T*;
template <class T> struct Kind { static constexpr int value = 0; };
template <class T> struct Kind<Pointer<T>> { static constexpr int value = 1; };
static_assert(Kind<int*>::value == 1);
static_assert(Kind<int>::value == 0);

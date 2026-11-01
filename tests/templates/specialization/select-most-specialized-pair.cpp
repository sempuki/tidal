// tidal-rules: TSP-15
// tidal-expect: accept
// tidal-note: Pair<int*, int*> matches all three partial specializations. Pair<T*, T*> is more specialized than each of the others.
template <class T, class U> struct Pair { static constexpr int value = 0; };
template <class T, class U> struct Pair<T*, U> { static constexpr int value = 1; };
template <class T> struct Pair<T, T> { static constexpr int value = 2; };
template <class T> struct Pair<T*, T*> { static constexpr int value = 3; };
static_assert(Pair<int*, char>::value == 1);
static_assert(Pair<int, int>::value == 2);
static_assert(Pair<int*, int*>::value == 3);

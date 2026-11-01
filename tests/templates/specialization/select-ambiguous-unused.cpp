// tidal-rules: TSP-16
// tidal-expect: accept
// tidal-note: The ambiguous partial specializations are never selected, so the program is valid.
template <class T, class U> struct Pair { static constexpr int value = 0; };
template <class T> struct Pair<T, int> { static constexpr int value = 1; };
template <class U> struct Pair<int, U> { static constexpr int value = 2; };
static_assert(Pair<char, int>::value == 1);
static_assert(Pair<int, char>::value == 2);
static_assert(Pair<char, char>::value == 0);

// tidal-rules: TPL-15 TPL-14
// tidal-expect: accept
// tidal-note: The second declaration adds a default for the first parameter. The two declarations together give both defaults.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T, class U = int> struct Pair;
template <class T = char, class U> struct Pair;
template <class T, class U> struct Pair { using first = T; using second = U; };
static_assert(same<Pair<>::first, char>);
static_assert(same<Pair<>::second, int>);

// tidal-rules: TARG-9
// tidal-expect: accept
// tidal-note: Missing trailing arguments come from the defaults, and an empty list takes every default.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T = int, class U = T*> struct Pair { using first = T; using second = U; };
static_assert(same<Pair<>::first, int>);
static_assert(same<Pair<>::second, int*>);
static_assert(same<Pair<char>::second, char*>);
static_assert(same<Pair<>, Pair<int, int*>>);

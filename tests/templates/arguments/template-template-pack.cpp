// tidal-rules: TARG-17
// tidal-expect: accept
// tidal-note: A template template parameter with a pack takes a template with any fixed number of type parameters.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T, class U> struct Pair {};
template <template <class...> class C> struct Apply { using type = C<int, char>; };
static_assert(same<Apply<Pair>::type, Pair<int, char>>);

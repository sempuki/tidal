// tidal-rules: TARG-17
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-note: The argument has one more parameter than the template template parameter, with a default. Clang 18 requires the parameter lists to have the same length (Q-0169).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T, class U = char> struct Pair {};
template <template <class> class C> struct Apply { using type = C<int>; };
static_assert(same<Apply<Pair>::type, Pair<int, char>>);

// tidal-rules: PACK-10
// tidal-expect: accept
// tidal-note: The expansion gives two arguments for the two parameters of Two, which are not packs.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class A, class B> struct Two {};
template <class... T> struct Hold { using type = Two<T...>; };
static_assert(same<Hold<int, char>::type, Two<int, char>>);

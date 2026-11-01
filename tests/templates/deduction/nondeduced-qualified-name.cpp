// tidal-rules: DED-13
// tidal-expect: accept
// tidal-note: The first parameter is in a non-deduced position. T comes from the second argument, and 1L then converts to int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> struct Id { using type = T; };
template <class T> Tag<T> first(typename Id<T>::type, T);
static_assert(same<decltype(first(1L, 2)), Tag<int>>);
template <int N> struct Fixed {};
template <int N> constexpr int next(Fixed<N + 1>, Fixed<N>) { return N; }
static_assert(next(Fixed<4>{}, Fixed<3>{}) == 3);

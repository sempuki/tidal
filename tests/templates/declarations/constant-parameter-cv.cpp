// tidal-rules: TPL-32
// tidal-expect: accept
// tidal-note: const on the parameter type is dropped, so the two declarations declare one template, and the name is an int prvalue.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <const int N> struct Count;
template <int N> struct Count { static constexpr int value = N; };
static_assert(Count<3>::value == 3);
template <const int N> constexpr auto get() { return N; }
static_assert(same<decltype(get<2>()), int>);

// tidal-rules: DED-12
// tidal-expect: accept
// tidal-note: N is declared auto. It takes its value from the argument of Wide<5>, and its type is long, the type of the parameter of Wide.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct Tag {};
template <long N> struct Wide {};
template <auto N> Tag<decltype(N)> width(Wide<N>);
static_assert(same<decltype(width(Wide<5>{})), Tag<long>>);

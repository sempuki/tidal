// tidal-rules: CTAD-10 CTAD-6
// tidal-expect: accept
// tidal-note: An empty list forms no aggregate candidate. The guide with no parameters gives Agg<T>, and T takes its default.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T = int> struct Agg { T v; };
Agg a{};
static_assert(same<decltype(a), Agg<int>>);

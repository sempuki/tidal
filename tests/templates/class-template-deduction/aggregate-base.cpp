// tidal-rules: CTAD-9 CTAD-10
// tidal-expect: accept
// tidal-note: The base Base<T> is an element of the aggregate candidate, as a member is. Its type is dependent, so its braces stay, and {1} gives T = int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Base { T v; };
template <class T> struct Wrap : Base<T> { T w; };
Wrap x{{1}, 2};
static_assert(same<decltype(x), Wrap<int>>);

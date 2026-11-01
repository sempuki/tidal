// tidal-rules: CTAD-4
// tidal-expect: accept
// tidal-note: Box<> has an argument list, so T takes its default, int, and 2.5 converts. Box alone deduces double.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T = int> struct Box { T v; constexpr Box(T x) : v(x) {} };
Box<> a(2);
Box b(2.5);
static_assert(same<decltype(a), Box<int>> && same<decltype(b), Box<double>>);

// tidal-rules: CTAD-8
// tidal-expect: accept
// tidal-note: In the guide from Hold(T&&), T&& is an rvalue reference, so a prvalue gives T = int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Hold { constexpr Hold(T&&) {} };
Hold h(1);
static_assert(same<decltype(h), Hold<int>>);

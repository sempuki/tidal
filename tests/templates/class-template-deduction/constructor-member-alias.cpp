// tidal-rules: CTAD-8
// tidal-expect: accept
// tidal-note: The member alias value_type of the class itself is T in the guide, so T deduces.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Wrap { using value_type = T; constexpr Wrap(value_type) {} };
Wrap w(1);
static_assert(same<decltype(w), Wrap<int>>);

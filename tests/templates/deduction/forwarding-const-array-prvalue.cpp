// tidal-rules: DED-8 VC-2
// tidal-expect: accept
// tidal-note: Gap review RV6-B-8. An array prvalue keeps the cv-qualifiers of its elements. For a braced prvalue of the alias type const int[2], decltype gives const int[2], and a forwarding reference deduces T as const int[2].
template <class A, class B> struct same { static constexpr bool value = false; };
template <class A> struct same<A, A> { static constexpr bool value = true; };
using Pair = const int[2];
static_assert(same<decltype(Pair{1, 2}), const int[2]>::value);
template <class T> constexpr bool deduced_const_array(T&&) { return same<T, const int[2]>::value; }
static_assert(deduced_const_array(Pair{1, 2}));

// tidal-rules: DED-24
// tidal-expect: accept
// tidal-note: After substitution, the function type is adjusted as a written one is. Top-level const and the array type of a parameter change.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> void take(T) {}
static_assert(same<decltype(&take<const int>), void (*)(int)>);
static_assert(same<decltype(&take<int[3]>), void (*)(int*)>);

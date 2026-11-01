// tidal-rules: DED-21
// tidal-expect: accept
// tidal-note: Every pair that names T gives the same value.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> apply(T (*)(T), T);
int keep(int);
static_assert(same<decltype(apply(keep, 1)), Tag<int>>);

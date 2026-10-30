// tidal-rules: AUTO-13
// tidal-expect: reject
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> concept Int = same<T, int>;
Int auto a = 1.0;

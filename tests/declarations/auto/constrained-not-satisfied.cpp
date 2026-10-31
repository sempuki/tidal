// tidal-rules: AUTO-16
// tidal-expect: reject
// tidal-error-clang: does not satisfy
// tidal-error-gcc: does not satisfy placeholder constraints
// tidal-error-edg: failed the type constraint
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> concept Int = same<T, int>;
Int auto a = 1.0;

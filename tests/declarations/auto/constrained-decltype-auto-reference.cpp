// tidal-rules: AUTO-16
// tidal-expect: reject
// tidal-error-clang: does not satisfy
// tidal-error-gcc: does not satisfy placeholder constraints
// tidal-error-edg: failed the type constraint
// tidal-note: decltype(auto) with a parenthesized name deduces int&, which does not satisfy a concept that asks for int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> concept Int = same<T, int>;
int n = 1;
Int decltype(auto) r = (n);

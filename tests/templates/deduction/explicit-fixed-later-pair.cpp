// tidal-rules: DED-3
// tidal-expect: accept
// tidal-note: The explicit argument fixes T. Both parameters of type T take no part in deduction, so 1 and 'c' convert to long. U is still to be deduced from the second argument.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T, class U> constexpr U mix(T a, U b, T c) { return b + a + c; }
static_assert(same<decltype(mix<long>(1, 2.0f, 'c')), float>);
static_assert(mix<long>(1, 2.0f, 'c') == 102.0f);

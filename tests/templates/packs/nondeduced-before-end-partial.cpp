// tidal-rules: PACK-12
// tidal-expect: accept
// tidal-note: In L<T..., U> the expansion is not last, so the second argument gives no values. U and T come from the first and third arguments, and the second argument must then be the same type.
template <class... T> struct L {};
template <class A, class B, class C> struct F {};
template <class> struct S { static constexpr int v = 0; };
template <class U, class... T> struct S<F<U, L<T..., U>, L<T...>>> { static constexpr int v = 1; };
static_assert(S<F<int, L<char, int>, L<char>>>::v == 1);
static_assert(S<F<int, L<char, long>, L<char>>>::v == 0);

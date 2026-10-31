// tidal-rules: TYN-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
typedef int A, *B, C[3], (*F)(int);
static_assert(same<B, int*>);
static_assert(same<C, int[3]>);
static_assert(same<F, int (*)(int)>);

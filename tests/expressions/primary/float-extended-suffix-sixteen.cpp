// tidal-rules: PRIM-22
// tidal-expect: accept
// tidal-note: The suffix f16 in both cases. Every compiler in the matrix accepts it, Clang included. The literal's type is none of the standard floating-point types.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto a = 1.5f16;
auto b = 2.5F16;
static_assert(same<decltype(a), decltype(b)>);
static_assert(!same<decltype(a), float> && !same<decltype(a), double> && !same<decltype(a), long double>);

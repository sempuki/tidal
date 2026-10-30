// tidal-rules: DCL-8
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f(int a[3], int b[], void c(), const int d);
static_assert(same<decltype(f), void(int*, int*, void (*)(), int)>);

// tidal-rules: COND-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct A {};
struct B { B(A) {} };
bool c = true;
static_assert(same<decltype(c ? A{} : B{A{}}), B>);

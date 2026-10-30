// tidal-rules: COND-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum E { A, B };
enum F { C };
E e = A;
static_assert(same<decltype(true ? e : e), E&>);
static_assert(same<decltype(true ? A : B), E>);

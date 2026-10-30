// tidal-rules: COND-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum E { A = 1 };
enum F { C = 2 };
static_assert(same<decltype(true ? A : C), int>);

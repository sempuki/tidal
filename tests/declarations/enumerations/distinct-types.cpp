// tidal-rules: ENUM-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum E { A };
enum F { B };
static_assert(!same<E, int>);
static_assert(!same<E, F>);
static_assert(same<decltype(A), E>);
static_assert(same<decltype(+A), int>);

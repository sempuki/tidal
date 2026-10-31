// tidal-rules: DTY-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum E { a };
enum class F : long { b };
static_assert(same<decltype(a), E>);
static_assert(same<decltype(F::b), F>);

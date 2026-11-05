// tidal-rules: ENUM-10
// tidal-expect: accept
// tidal-note: With a fixed underlying type, the first enumerator has the underlying type inside the list, also without an initializer.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Small : short { zero, check = same<decltype(zero), short> ? 1 : 2 };
static_assert(check == 1);

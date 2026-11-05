// tidal-rules: ENUM-10
// tidal-expect: accept
// tidal-note: The first enumerator has no initializer, so it has no enumerator before it to take a type from. Inside the list it has the type int on every matrix compiler.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Start { zero, check = same<decltype(zero), int> ? 1 : 2 };
static_assert(check == 1);

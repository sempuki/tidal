// tidal-rules: PRIM-1
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-note: Q-0380. The hexadecimal literal with the z suffix does not fit the signed type that matches std::size_t, and fits std::size_t. Clang and EDG give the literal the type std::size_t. GCC gives it the signed type with no diagnostic, so the assertion fails.
// tidal-error-gcc: static assertion failed
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(0xFFFF'FFFF'FFFF'FFFFz), decltype(sizeof 0)>);
static_assert(0xFFFF'FFFF'FFFF'FFFFz == static_cast<decltype(sizeof 0)>(-1));

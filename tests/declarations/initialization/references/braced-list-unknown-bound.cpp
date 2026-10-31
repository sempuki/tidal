// tidal-rules: REF-14
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: Q-0149. Clang gives the reference the bound of the list. GCC and EDG keep the declared type.
template<class A, class B> constexpr bool same = false;
template<class A> constexpr bool same<A, A> = true;

int (&&values)[]{1, 2, 3};
static_assert(same<decltype(values), int (&&)[]>);

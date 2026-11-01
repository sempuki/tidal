// tidal-rules: TPL-46
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: Two uses of one specialization give one closure type. Clang gives two closure types (Q-0200).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> using Closure = decltype([] {});
static_assert(same<Closure<int>, Closure<int>>);

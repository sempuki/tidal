// tidal-rules: TPL-46
// tidal-expect: accept
// tidal-note: The closure types of two specializations with different arguments differ, though the lambda does not use the parameter.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> using Closure = decltype([] {});
static_assert(!same<Closure<int>, Closure<char>>);

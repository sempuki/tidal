// tidal-rules: LAM-18
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-gcc-14: reject
// tidal-error-clang: drops .const. qualifier
// tidal-error-gcc: discards qualifiers
// tidal-note: Gap review 2026-10-31. Q-0092. GCC 16 and EDG find the capture in the trailing return type. Clang and GCC 14 find the outer variable.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f() { int x = 1; auto l = [x]() -> decltype((x)) { return x; }; static_assert(same<decltype(l()), const int&>); }

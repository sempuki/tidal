// tidal-rules: JMP-10
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Decision 0059, item RV5, point RV5-A-16. Q-0422. A structured binding named in parentheses in a return statement stays an lvalue, so decltype(auto) gives int&. Clang and GCC agree. EDG gives another type.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct Pair { int a; int b; };
decltype(auto) first() {
  auto [a, b] = Pair{1, 2};
  (void)b;
  return (a);
}
static_assert(same<decltype(first()), int&>);

// tidal-rules: CTAD-1
// tidal-expect: accept
// tidal-note: W is a template template parameter. After substitution W is Box, and every compiler deduces Box<int> (Q-0275).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { constexpr Box(T) {} };
template <template <class> class W> constexpr bool check() {
  W w(1);
  return same<decltype(w), W<int>>;
}
static_assert(check<Box>());

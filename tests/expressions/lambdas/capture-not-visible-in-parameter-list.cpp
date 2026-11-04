// tidal-rules: LAM-18
// tidal-expect: accept
// tidal-note: Q-0092. In the parameter list, x names the enclosing variable, so decltype((x)) is int& and not const int&.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f() {
  int x = 1;
  auto l = [x](decltype((x)) p, int q = sizeof(x)) { static_assert(same<decltype(p), int&>); return q; };
  (void)l;
}

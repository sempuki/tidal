// tidal-rules: DCL-13
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct W { int v; };
void f() {
  W w(W());
  static_assert(same<decltype(w), W(W (*)())>);
}

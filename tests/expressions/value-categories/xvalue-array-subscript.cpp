// tidal-rules: VC-3
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0034. EDG gives an lvalue for a subscript of an array xvalue or prvalue. Clang and GCC give an xvalue.
// tidal-note: Gap review 2026-10-31 (third).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using A = int[2];
void f(A& a) {
  static_assert(same<decltype(static_cast<A&&>(a)[0]), int&&>);
  static_assert(same<decltype(A{1, 2}[0]), int&&>);
}

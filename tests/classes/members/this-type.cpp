// tidal-rules: CLS-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S {
  void f() { static_assert(same<decltype(this), S*>); }
  void g() const { static_assert(same<decltype(this), const S*>); }
  void h() volatile { static_assert(same<decltype(this), volatile S*>); }
};

// tidal-rules: AUTO-12
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f(int* p) {
  if (auto q = p; auto& r = *q) { static_assert(same<decltype(r), int&>); }
  for (auto i = 0u; i < 2; ++i) { static_assert(same<decltype(i), unsigned>); }
}

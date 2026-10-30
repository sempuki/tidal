// tidal-rules: AUTO-14
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S {
  inline static auto n = 3;
  static constexpr auto m = 2.5;
};
static_assert(same<decltype(S::n), int>);
static_assert(same<decltype(S::m), const double>);

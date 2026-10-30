// tidal-rules: SB-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int a; double b; const char c; };
constexpr S s{1, 2.5, 3};
constexpr bool check() {
  auto [p, q, r] = s;
  return p == 1 && q == 2.5 && r == 3;
}
static_assert(check());

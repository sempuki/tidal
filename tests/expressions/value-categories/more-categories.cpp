// tidal-rules: VC-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 0;
enum E { A };
struct S { void m() { static_assert(same<decltype((this)), S*>); } };
static_assert(same<decltype((static_cast<int&>(x))), int&>);
static_assert(same<decltype((A)), E>);
static_assert(same<decltype((static_cast<long>(x))), long>);
static_assert(same<decltype((0, x)), int&>);
static_assert(same<decltype((x, 0)), int>);

// tidal-rules: CONV-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { long long small : 3; long long wide : 40; unsigned full : 32; };
S s;
static_assert(same<decltype(s.small + 0), int>);
static_assert(same<decltype(s.wide + 0), long long>);
static_assert(same<decltype(+s.full), unsigned>);

// tidal-rules: DCL-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int (x) = 3;
int (*(p)) = &x;
static_assert(same<decltype(p), int*>);

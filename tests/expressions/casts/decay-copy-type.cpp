// tidal-rules: CAST-13
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int arr[3];
const int ci = 1;
static_assert(same<decltype(auto(arr)), int*>);
static_assert(same<decltype(auto{ci}), int>);

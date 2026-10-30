// tidal-rules: AUTO-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int arr[3];
void fn();
auto p = arr;
auto q = fn;
auto s = "hi";
static_assert(same<decltype(p), int*>);
static_assert(same<decltype(q), void (*)()>);
static_assert(same<decltype(s), const char*>);

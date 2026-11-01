// tidal-rules: AUTO-8
// tidal-expect: accept
// tidal-note: OVL-20 leaves one function, so the variable is a pointer to that function.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
void once(int);
template <class T> void each(T);
auto p = once;
auto q = each<long>;
static_assert(same<decltype(p), void (*)(int)>);
static_assert(same<decltype(q), void (*)(long)>);

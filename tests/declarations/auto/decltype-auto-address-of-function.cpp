// tidal-rules: AUTO-11
// tidal-expect: accept
// tidal-note: &once is a prvalue of pointer type, so decltype(auto) gives a pointer and no function type.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
void once(int);
decltype(auto) p = &once;
static_assert(same<decltype(p), void (*)(int)>);

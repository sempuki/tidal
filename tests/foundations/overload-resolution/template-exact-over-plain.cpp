// tidal-rules: OVL-27 OVL-25
// tidal-expect: accept
// tidal-note: The specialization f<long> matches exactly and f(int) needs a conversion. The sequences decide before OVL-27.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One f(int);
template <class T> Two f(T);
static_assert(same<decltype(f(1L)), Two>);

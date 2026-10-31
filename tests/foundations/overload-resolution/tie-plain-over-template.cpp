// tidal-rules: OVL-27
// tidal-expect: accept
// tidal-note: The specialization f<int> has the same sequence as f(int), so no argument decides. No conversion function, inherited constructor, or rewritten candidate is involved. The plain function wins.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One f(int);
template <class T> Two f(T);
static_assert(same<decltype(f(1)), One>);

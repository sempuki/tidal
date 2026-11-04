// tidal-rules: OVL-41
// tidal-expect: accept
// tidal-note: Both templates have the parameter T and give the same specialization type, so the sequences tie and both are templates. The constrained template is the more specialized one.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
template <class T> One f(T);
template <class T> requires (sizeof(T) > 1) Two f(T);
static_assert(same<decltype(f(1)), Two>);
static_assert(same<decltype(f('c')), One>);

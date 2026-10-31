// tidal-rules: OVL-28
// tidal-expect: accept
// tidal-note: Both specializations take int* with the identity, and both are templates, so OVL-25 to OVL-27 do not decide. The one from f(T*) is more specialized.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
template <class T> One f(T);
template <class T> Two f(T*);
int* p = nullptr;
static_assert(same<decltype(f(p)), Two>);

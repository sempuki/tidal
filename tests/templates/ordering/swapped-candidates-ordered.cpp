// tidal-rules: ORD-7
// tidal-expect: accept
// tidal-note: Both candidates for w == v are swapped. Reversing the parameters of both keeps the second more specialized. The case does not depend on Q-0211.
template <class T> struct W {};
template <class T> struct V {};
template <class T, class U> constexpr bool operator==(const V<U>&, const W<T>&) { return false; }
template <class T> constexpr bool operator==(const V<T*>&, const W<T*>&) { return true; }
constexpr W<int*> w;
constexpr V<int*> v;
static_assert(w == v);

// tidal-rules: TSP-16
// tidal-expect: reject
// tidal-error-clang: ambiguous partial specializations of
// tidal-error-gcc: ambiguous template instantiation for
// tidal-error-edg: more than one partial specialization matches the template
// tidal-note: Pair<int, int> matches both partial specializations, and neither is more specialized.
template <class T, class U> struct Pair {};
template <class T> struct Pair<T, int> {};
template <class U> struct Pair<int, U> {};
Pair<int, int> p;

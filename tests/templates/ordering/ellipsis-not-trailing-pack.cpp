// tidal-rules: ORD-12 ORD-15
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance
// tidal-note: Only the first parameter takes part in ordering, and the two templates tie. An ellipsis is not a function parameter pack, so ORD-12 does not prefer pick(T), and the call is ambiguous.
template <class T> int pick(T);
template <class T> int pick(T, ...);
int r = pick(1);

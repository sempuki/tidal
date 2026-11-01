// tidal-rules: ORD-1 ORD-15
// tidal-expect: reject
// tidal-error-clang: call to .* is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Neither template can be deduced from the other's invented types, so neither is more specialized and the call is ambiguous.
template <class T> void put(T, int);
template <class T> void put(int, T);
void use() { put(1, 1); }

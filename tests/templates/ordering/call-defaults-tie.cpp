// tidal-rules: ORD-5 ORD-15
// tidal-expect: reject
// tidal-error-clang: call to .* is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The defaulted parameter takes no part, so both templates compare as (T) and neither is more specialized.
template <class T> void f(T, int = 0);
template <class T> void f(T);
void use() { f(1); }

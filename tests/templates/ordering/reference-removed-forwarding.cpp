// tidal-rules: ORD-2 ORD-15
// tidal-expect: reject
// tidal-error-clang: call to .* is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: After the reference is removed, T&& and T are both T, so neither template is more specialized. ORD-10 does not apply, because only one of the two types was a reference.
template <class T> void take(T);
template <class T> void take(T&&);
int n = 0;
void use() { take(n); }

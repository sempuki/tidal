// tidal-rules: ORD-2 ORD-15
// tidal-expect: reject
// tidal-error-clang: call to .* is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: After the reference and const are removed, both parameter types are T, so neither template is more specialized.
template <class T> void take(T);
template <class T> void take(const T&);
int n = 0;
void use() { take(n); }

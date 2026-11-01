// tidal-rules: ORD-10 ORD-15
// tidal-expect: reject
// tidal-error-clang: call to .* is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Both parameters of both templates become int&. The first pair prefers the first template and the second pair the second, so neither is more specialized.
template <class T, class U> void f(T&, U&&);
template <class T, class U> void f(T&&, U&);
int n = 0;
void use() { f(n, n); }

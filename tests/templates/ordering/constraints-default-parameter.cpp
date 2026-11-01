// tidal-rules: ORD-13 ORD-15
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: call to .* is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-note: The constrained template has a second parameter with a default argument. The parameter lists differ in length, so the constraint does not decide. EDG lets the constraint decide (Q-0146).
template <class T> void f(T, int = 0) requires (sizeof(T) > 1);
template <class T> void f(T);
void use() { f(1); }

// tidal-rules: ORD-13 ORD-15
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: call to .* is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-note: The function parameters are the same, but the template heads differ in length, so the constraint does not decide. EDG lets the constraint decide (Q-0146).
template <class T> void f(T);
template <class T, class U = int> requires (sizeof(T) > 1) void f(T);
void use() { f(1); }

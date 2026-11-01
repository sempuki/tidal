// tidal-rules: ORD-12 ORD-15
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: call to .* is ambiguous
// tidal-note: The first template has a parameter with a default argument where the second has its pack. That parameter counts, so ORD-12 does not decide, and the call is ambiguous. GCC and EDG count only parameters with call arguments and choose the first template (Q-0216).
template <class T> void f(T, T = T());
template <class T, class... U> void f(T, U...);
void use() { f(1); }

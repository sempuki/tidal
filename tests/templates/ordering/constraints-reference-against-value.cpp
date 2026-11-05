// tidal-rules: ORD-13 ORD-15
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-note: The parameters const T& and T tie in ordering, because ORD-2 removes the reference and the const. ORD-13 compares the declared types, which differ, so the constraint does not decide and the call is ambiguous. EDG lets the constraint decide (Q-0146).
template <class T> constexpr int f(const T&) requires (sizeof(T) > 1) { return 1; }
template <class T> constexpr int f(T) { return 2; }
int n = 0;
int r = f(n);

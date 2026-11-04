// tidal-rules: CTOR-17
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: binds to a temporary object whose lifetime would be shorter
// tidal-note: Q-0084. CTOR-17 rejects a reference member bound to a temporary that ends with the constructor. Clang rejects. GCC (with a warning) and EDG accept.
struct S { const int& r; S() : r(42) {} };
S s;

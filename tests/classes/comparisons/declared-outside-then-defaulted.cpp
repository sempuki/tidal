// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: already declared outside the class
// tidal-error-edg: must be the first declaration
// tidal-note: Q-0061. GCC accepts defaulting a friend comparison that was declared earlier outside the class. Clang and EDG reject it.
struct S;
bool operator==(const S&, const S&);
struct S { int x; friend bool operator==(const S&, const S&) = default; };

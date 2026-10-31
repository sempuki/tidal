// tidal-rules: PFX-15
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: cannot pass object of non-trivial type .S. through variadic function
// tidal-note: Gap review 2026-10-31. Conditionally supported. GCC and EDG support it. Clang rejects it.
struct S { S() {} S(const S&) {} };
void v(int, ...);
void f() { S s; v(0, s); }

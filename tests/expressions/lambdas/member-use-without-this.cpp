// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-error-clang: .this. cannot be implicitly captured
// tidal-error-gcc: .this. was not captured
// tidal-error-edg: this. cannot be referenced in a lambda body
// tidal-note: Gap review 2026-10-31.
struct S { int v; int f() { return [] { return v; }(); } };

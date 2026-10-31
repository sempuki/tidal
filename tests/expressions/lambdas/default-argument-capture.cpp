// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-error-clang: lambda expression in default argument cannot capture
// tidal-error-gcc: default argument .* uses local variable
// tidal-error-edg: cannot be captured
// tidal-note: Gap review 2026-10-31.
void f(int x) { void g(int = [x] { return x; }()); }

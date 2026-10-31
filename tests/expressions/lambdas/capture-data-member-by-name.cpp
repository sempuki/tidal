// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-error-clang: (does not name a variable|cannot appear in capture list as it is not a variable)
// tidal-error-gcc: capture of non-variable
// tidal-error-edg: is not a variable
// tidal-note: Gap review 2026-10-31.
struct S { int v; void f() { auto l = [v] { return v; }; } };

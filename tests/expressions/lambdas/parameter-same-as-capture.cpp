// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-error-clang: parameter cannot shadow an explicitly captured entity
// tidal-error-gcc: previously declared as a capture
// tidal-error-edg: lambda parameter cannot hide an explicit capture
// tidal-note: Gap review 2026-10-31.
void f() { int x = 1; auto l = [x](int x) { return x; }; }

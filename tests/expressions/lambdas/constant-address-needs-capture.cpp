// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-error-clang: cannot be implicitly captured
// tidal-error-gcc: .c. is not captured
// tidal-error-edg: must be an lvalue
// tidal-note: Gap review 2026-10-31.
void f() { const int c = 4; auto l = [] { const int* p = &c; return *p; }; }

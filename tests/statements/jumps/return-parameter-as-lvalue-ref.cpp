// tidal-rules: JMP-15
// tidal-expect: reject
// tidal-error-clang: non-const lvalue reference to type .int. cannot bind to a temporary
// tidal-error-gcc: cannot bind non-const lvalue reference of type .int&. to an rvalue
// tidal-error-edg: reference to non-const must be an lvalue
// tidal-note: Gap review 2026-10-30.
int& keep(int v) { return v; }

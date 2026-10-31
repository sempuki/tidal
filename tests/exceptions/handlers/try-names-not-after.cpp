// tidal-rules: TRY-14
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .inside
// tidal-error-gcc: inside. was not declared in this scope
// tidal-error-edg: identifier .inside. is undefined
void g() { try { int inside = 1; } catch (...) { } int w = inside; }

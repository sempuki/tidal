// tidal-rules: TRY-14
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .v
// tidal-error-gcc: v. was not declared in this scope
// tidal-error-edg: identifier .v. is undefined
void g() { try { } catch (int v) { } int w = v; }

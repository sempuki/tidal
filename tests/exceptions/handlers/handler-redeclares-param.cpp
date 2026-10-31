// tidal-rules: TRY-14
// tidal-expect: reject
// tidal-error-clang: redefinition of .v
// tidal-error-gcc: redeclaration of .int v
// tidal-error-edg: v. has already been declared in the current scope
void g() { try { } catch (int v) { int v = 2; } }

// tidal-rules: TRY-7
// tidal-expect: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-gcc: jump to label
// tidal-error-edg: transfer of control into an exception handler
void g() { goto inside; try { } catch (...) { inside: ; } }

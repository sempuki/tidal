// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: cannot catch pointer to incomplete type
// tidal-error-gcc: invalid use of array with unspecified bounds
// tidal-error-edg: incomplete type
void g() { try { } catch (int (*)[]) { } }

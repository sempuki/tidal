// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: cannot catch incomplete type .void
// tidal-error-gcc: declared void
// tidal-error-edg: incomplete type .void. is not allowed
void g() { try { } catch (void) { } }

// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: cannot catch reference to incomplete type
// tidal-error-gcc: array with unspecified bounds
// tidal-error-edg: reference to incomplete type
void g() { try { } catch (int (&)[]) { } }

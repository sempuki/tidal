// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: cannot catch pointer to incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: pointer or reference to incomplete type
struct I;
void g() { try { } catch (I*) { } }

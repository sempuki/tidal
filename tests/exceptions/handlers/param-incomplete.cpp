// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: cannot catch incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: incomplete type .I. is not allowed
struct I;
void g() { try { } catch (I) { } }

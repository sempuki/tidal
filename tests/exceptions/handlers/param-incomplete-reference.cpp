// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: cannot catch reference to incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: pointer or reference to incomplete type
struct Pending;
void g() { try { } catch (const Pending&) { } }

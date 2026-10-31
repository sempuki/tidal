// tidal-rules: CPY-6
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .const S
// tidal-error-gcc: discards qualifiers
// tidal-error-edg: no suitable copy constructor
struct M { M() {} M(M&) {} };
struct S { M m; };
const S a;
S b(a);

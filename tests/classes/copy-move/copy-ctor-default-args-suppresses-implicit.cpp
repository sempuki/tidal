// tidal-rules: CPY-1
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .S
// tidal-error-gcc: discards qualifiers
// tidal-error-edg: no instance of constructor .S::S
struct S { S() {} S(S&, int = 0) {} };
const S a;
S b(a);

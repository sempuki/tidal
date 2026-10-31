// tidal-rules: CPY-24
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .S
// tidal-error-gcc: no matching function for call to .S::S
// tidal-error-edg: no suitable copy constructor
struct S { S() {} explicit S(const S&) {} };
S a;
S b = a;

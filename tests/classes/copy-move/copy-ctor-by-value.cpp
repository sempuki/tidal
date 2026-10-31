// tidal-rules: CPY-15
// tidal-expect: reject
// tidal-error-clang: must pass its first argument by reference
// tidal-error-gcc: invalid constructor
// tidal-error-edg: copy constructor .* may not have a parameter of type
struct S { S() {} S(S) {} };

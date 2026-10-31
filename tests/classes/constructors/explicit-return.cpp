// tidal-rules: CTOR-9
// tidal-expect: reject
// tidal-error-clang: no viable conversion from returned value of type .int. to function return type .S
// tidal-error-gcc: could not convert .1. from .int. to .S
// tidal-error-edg: no suitable constructor exists to convert from .int. to .S
struct S { explicit S(int) {} };
S make() { return 1; }

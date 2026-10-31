// tidal-rules: VOL-11
// tidal-expect: reject
// tidal-error-clang: from .volatile S.|no matching constructor for initialization of .S
// tidal-error-gcc: S\(volatile S&\)
// tidal-error-edg: no suitable copy constructor
struct S { int v; };
volatile S a{};
S b = a;

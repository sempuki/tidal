// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-clang: must have at least one parameter of class or enumeration type
// tidal-error-gcc: must have an argument of class or enumerated type
// tidal-error-edg: requires a parameter with class or enum type
int operator+(int a, int b);

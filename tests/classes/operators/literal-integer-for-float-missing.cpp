// tidal-rules: OPR-16
// tidal-expect: reject
// tidal-error-clang: no matching literal operator
// tidal-error-gcc: unable to find numeric literal operator
// tidal-error-edg: user-defined literal operator not found
unsigned long long operator""_u(unsigned long long n) { return n; }
auto x = 1.5_u;

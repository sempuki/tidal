// tidal-rules: PFX-5
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .f.
// tidal-error-gcc: too many arguments to function
// tidal-error-edg: too many arguments in function call
int f(int a);
int g() { return f(1, 2); }

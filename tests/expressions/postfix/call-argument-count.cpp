// tidal-rules: PFX-5
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .f.
// tidal-error-gcc: too few arguments to function
// tidal-error-edg: too few arguments in function call
int f(int a, int b);
int g() { return f(1); }

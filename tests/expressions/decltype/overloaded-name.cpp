// tidal-rules: DTY-4
// tidal-expect: reject
// tidal-error-clang: reference to overloaded function could not be resolved
// tidal-error-gcc: cannot resolve address of overloaded function
// tidal-error-edg: which instance of overloaded function
int f(int);
int f(double);
decltype(f) x;

// tidal-rules: DEFARG-5
// tidal-expect: reject
// tidal-error-clang: too few arguments to function call
// tidal-error-gcc: too few arguments to function
// tidal-error-edg: too few arguments in function call
int f(int a = 1);
int (&r)(int) = f;
int x = r();          // the reference carries no default argument

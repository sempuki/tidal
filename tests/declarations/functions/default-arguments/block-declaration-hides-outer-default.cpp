// tidal-rules: DEFARG-2 PFX-5
// tidal-expect: reject
// tidal-error-clang: too few arguments to function call
// tidal-error-gcc: too few arguments to function
// tidal-error-edg: too few arguments in function call
int f(int a = 1);
int g() {
  int f(int a);       // a block declaration with no default
  return f();         // the namespace default is hidden
}

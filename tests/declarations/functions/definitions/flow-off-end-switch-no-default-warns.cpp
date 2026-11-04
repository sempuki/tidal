// tidal-rules: FN-7
// tidal-expect: reject
// tidal-flags: -Werror -Wreturn-type
// tidal-error-clang: does not return a value
// tidal-error-gcc: control reaches end of non-void function|no return statement
// tidal-note: A switch without a default label can complete, whatever its cases.
int f(int n) {
  switch (n) {
    case 0: return 1;
    case 1: return 2;
  }
}

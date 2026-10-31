// tidal-rules: FN-7
// tidal-expect: reject
// tidal-error-clang: non-void function does not return a value in all control paths
// tidal-error-gcc: control reaches end of non-void function
// tidal-flags: -Werror -Wreturn-type
int f(bool b) {
  if (b) return 1;
}

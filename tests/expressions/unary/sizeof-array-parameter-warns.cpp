// tidal-rules: UN-19
// tidal-expect: reject
// tidal-error-clang: sizeof on array function parameter
// tidal-error-gcc: sizeof. on array function parameter
// tidal-flags: -Werror -Wsizeof-array-argument
unsigned long f(int a[10]) {
  return sizeof(a);
}

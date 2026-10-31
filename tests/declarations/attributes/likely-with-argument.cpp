// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: cannot have an argument list
// tidal-error-gcc: does not take any arguments
// tidal-error-edg: does not take arguments
int pick(int v) {
  if (v > 0) [[likely(1)]] return 1;
  return 0;
}

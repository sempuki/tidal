// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: cannot have an argument list
// tidal-error-gcc: does not take any arguments
// tidal-error-edg: does not take arguments
struct Empty {};
struct Holder {
  [[no_unique_address(1)]] Empty e;
  int v;
};

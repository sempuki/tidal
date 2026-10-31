// tidal-rules: RET-3
// tidal-expect: reject
// tidal-error-clang: deduced return type cannot be used before it is defined
// tidal-error-gcc: before deduction of .auto.
// tidal-error-edg: cannot deduce the return type
struct Counter {
  int total() { return step() + 1; }
  auto step() { return 1; }
};
int use() { Counter c; return c.total(); }

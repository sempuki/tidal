// tidal-rules: CX-6
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: call to non-.constexpr. function .int read\(\)
// tidal-error-edg: must have a constant value
int read() { return 1; }
constexpr int f() { return read(); }
static_assert(f() == 1);

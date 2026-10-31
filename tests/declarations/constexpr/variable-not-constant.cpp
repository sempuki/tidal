// tidal-rules: CX-1
// tidal-expect: reject
// tidal-error-clang: constexpr variable .n. must be initialized by a constant expression
// tidal-error-gcc: call to non-.constexpr. function .int read\(\)
// tidal-error-edg: must have a constant value
int read();
constexpr int n = read();

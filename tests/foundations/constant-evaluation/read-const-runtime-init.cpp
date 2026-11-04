// tidal-rules: CE-3
// tidal-expect: reject
// tidal-error-gcc: value of .n. is not usable in a constant expression
// tidal-error-clang: initializer of .n. is not a constant expression
// tidal-error-edg: value of variable .n. .* cannot be used as a constant
int read();
const int n = read();
static_assert(n == 3);

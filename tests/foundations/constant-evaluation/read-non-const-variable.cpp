// tidal-rules: CE-3
// tidal-expect: reject
// tidal-error-gcc: value of .n. is not usable in a constant expression
// tidal-error-clang: read of non-const variable
// tidal-error-edg: value of variable .n. .* cannot be used as a constant
int n = 3;
static_assert(n == 3);

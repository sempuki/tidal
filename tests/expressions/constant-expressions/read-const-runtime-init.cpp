// tidal-rules: CE-3
// tidal-expect: reject
// tidal-error-gcc: value of .n. is not usable in a constant expression
int read();
const int n = read();
static_assert(n == 3);

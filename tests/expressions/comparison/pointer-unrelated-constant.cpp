// tidal-rules: CMP-8
// tidal-expect: reject
// tidal-error-gcc: \(& a\) < \(& b\)\). is not a constant expression
// tidal-error-clang: comparison between .* has unspecified value
// tidal-error-edg: pointer values cannot be compared
int a, b;
static_assert((&a < &b) || (&b < &a));

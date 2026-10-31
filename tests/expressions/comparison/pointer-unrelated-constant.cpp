// tidal-rules: CMP-8
// tidal-expect: reject
// tidal-error-gcc: \(& a\) < \(& b\)\). is not a constant expression
int a, b;
static_assert((&a < &b) || (&b < &a));

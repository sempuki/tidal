// tidal-rules: CMP-8
// tidal-expect: reject
int a, b;
static_assert((&a < &b) || (&b < &a));

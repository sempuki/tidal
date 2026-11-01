// tidal-rules: CMP-11
// tidal-expect: accept
int a[2], b[2];
static_assert(a == a);
static_assert(a != b);
static_assert(!(a != a));

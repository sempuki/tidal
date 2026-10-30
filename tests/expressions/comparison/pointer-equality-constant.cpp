// tidal-rules: CMP-15
// tidal-expect: accept
int a, b;
static_assert(&a != &b);
static_assert(&a == &a);
static_assert(&a != nullptr);

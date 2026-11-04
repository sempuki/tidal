// tidal-rules: CE-1
// tidal-expect: accept
int a(); int b();
static_assert(&a != &b);
static_assert(&a == &a);

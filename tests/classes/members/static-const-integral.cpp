// tidal-rules: CLS-3
// tidal-expect: accept
struct S { static const int n = 4; int buf[n]; };
static_assert(S::n == 4 && sizeof(S) == 4 * sizeof(int));

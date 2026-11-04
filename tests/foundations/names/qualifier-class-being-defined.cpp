// tidal-rules: NAM-16
// tidal-expect: accept
// tidal-note: The class C is not complete in the array bound. C::n finds the member n, which is declared before the use.
struct C { static const int n = 3; int a[C::n]; };
static_assert(sizeof(C) == 3 * sizeof(int));

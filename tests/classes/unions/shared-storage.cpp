// tidal-rules: UNI-1
// tidal-expect: accept
union U { char c; int i; double d; };
static_assert(sizeof(U) >= sizeof(double));
static_assert(alignof(U) >= alignof(double));

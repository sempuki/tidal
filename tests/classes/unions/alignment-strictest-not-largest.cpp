// tidal-rules: UNI-1
// tidal-expect: accept
union U { char buf[16]; double d; };
static_assert(alignof(U) == alignof(double));
static_assert(sizeof(U) == 16);

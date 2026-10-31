// tidal-rules: ALN-1
// tidal-expect: accept
struct alignas(double) S { char c; };
static_assert(alignof(S) == alignof(double));

// tidal-rules: ALN-1
// tidal-expect: accept
struct alignas(0) S { int v; };
static_assert(alignof(S) == alignof(int));

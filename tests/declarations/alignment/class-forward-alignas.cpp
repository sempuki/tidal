// tidal-rules: ALN-1
// tidal-expect: accept
struct alignas(16) S;
struct alignas(16) S { int v; };
static_assert(alignof(S) == 16);

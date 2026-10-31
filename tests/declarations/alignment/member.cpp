// tidal-rules: ALN-1
// tidal-expect: accept
struct S { char c; alignas(8) int v; };
static_assert(alignof(S) == 8);
static_assert(sizeof(S) == 16);

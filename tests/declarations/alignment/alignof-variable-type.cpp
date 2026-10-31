// tidal-rules: ALN-1
// tidal-expect: accept
alignas(32) int x;
static_assert(alignof(decltype(x)) == alignof(int));

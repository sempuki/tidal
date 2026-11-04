// tidal-rules: ALN-1
// tidal-expect: accept
union alignas(8) Small { char c; };
union alignas(16) Later;                // a forward declaration
union alignas(16) Later { char c; };
static_assert(alignof(Small) == 8 && sizeof(Small) == 8);
static_assert(alignof(Later) == 16);

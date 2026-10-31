// tidal-rules: ALN-1
// tidal-expect: accept
struct Inner { alignas(64) char c; };
struct Outer { char x; Inner in; };
static_assert(alignof(Outer) == 64);
static_assert(sizeof(Outer) == 128);

// tidal-rules: ALN-1
// tidal-expect: accept
struct alignas(32) Line { char data[4]; };
static_assert(alignof(Line) == 32);
static_assert(sizeof(Line) == 32);

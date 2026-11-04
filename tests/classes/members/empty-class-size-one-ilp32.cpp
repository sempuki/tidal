// tidal-rules: CLS-11
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The same size as on x86-64 (empty-class-size-one.cpp).
struct Empty {};
static_assert(sizeof(Empty) == 1);
static_assert(sizeof(Empty[4]) == 4);

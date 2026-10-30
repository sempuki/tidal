// tidal-rules: ATTR-9
// tidal-expect: accept
struct Empty {};
struct Plain { Empty e; int v; };
struct Packed { [[no_unique_address]] Empty e; int v; };
static_assert(sizeof(Plain) > sizeof(int));
static_assert(sizeof(Packed) == sizeof(int));

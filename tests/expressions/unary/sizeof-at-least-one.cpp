// tidal-rules: UN-12
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct Empty {};
struct Pad { char c; int i; };
static_assert(sizeof(Empty) >= 1);
static_assert(sizeof(Pad) >= sizeof(char) + sizeof(int));

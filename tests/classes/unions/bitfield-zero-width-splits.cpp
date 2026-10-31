// tidal-rules: BF-9
// tidal-expect: accept
struct S { unsigned a : 3; unsigned : 0; unsigned b : 3; };
static_assert(sizeof(S) == 2 * sizeof(unsigned));

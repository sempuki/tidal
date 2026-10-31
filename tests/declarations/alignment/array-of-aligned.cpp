// tidal-rules: ALN-1
// tidal-expect: accept
struct alignas(16) S { int v; };
S arr[3];
static_assert(sizeof(arr) == 48);

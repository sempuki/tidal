// tidal-rules: ENUM-5
// tidal-expect: accept
enum Small { S = 1 };
static_assert(sizeof(Small) == sizeof(int));

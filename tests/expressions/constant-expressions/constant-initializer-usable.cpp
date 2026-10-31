// tidal-rules: CE-24 CV-8
// tidal-expect: accept
constexpr int twice(int v) { return 2 * v; }
const int limit = twice(4);
static_assert(limit == 8);
int buffer[limit];

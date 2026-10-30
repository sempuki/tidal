// tidal-rules: DEL-1
// tidal-expect: reject
int f() = delete;
using T = decltype(f());
static_assert(sizeof(f()) == sizeof(int));

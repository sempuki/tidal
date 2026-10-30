// tidal-rules: UN-7
// tidal-expect: reject
constexpr int read(int* p) { return *p; }
static_assert(read(nullptr) == 0);

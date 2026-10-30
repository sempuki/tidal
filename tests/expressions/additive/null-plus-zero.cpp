// tidal-rules: ADD-8
// tidal-expect: accept
constexpr int* p = nullptr;
constexpr int* q = p + 0;
static_assert(q == nullptr);
static_assert(p - p == 0);

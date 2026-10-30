// tidal-rules: ADD-10
// tidal-expect: reject
constexpr int* p = nullptr;
constexpr int* q = p + 1;

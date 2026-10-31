// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-error-gcc: arithmetic involving a null pointer
constexpr int* p = nullptr;
constexpr int* q = p + 1;

// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-error-gcc: arithmetic involving a null pointer
// tidal-error-clang: pointer arithmetic on null pointer
// tidal-error-edg: operation not allowed on null pointer
constexpr int* p = nullptr;
constexpr int* q = p + 1;

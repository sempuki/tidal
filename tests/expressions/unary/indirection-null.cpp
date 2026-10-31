// tidal-rules: UN-7
// tidal-expect: reject
// tidal-error-gcc: dereferencing a null pointer
// tidal-error-clang: dereferenced null pointer|dereferencing a null pointer
constexpr int read(int* p) { return *p; }
static_assert(read(nullptr) == 0);

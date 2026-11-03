// tidal-rules: CAST-8
// tidal-expect: reject
// tidal-error-clang: reinterpret_cast is not allowed in a constant expression
// tidal-error-gcc: reinterpret_cast. is not a constant expression
// tidal-error-edg: is invalid in constant-expression evaluation
// tidal-note: The reinterpret_cast is in a constexpr function that constant evaluation calls.
constexpr char first(const int* p) { return *reinterpret_cast<const char*>(p); }
constexpr int value = 1;
static_assert(first(&value) == 1 || first(&value) == 0);

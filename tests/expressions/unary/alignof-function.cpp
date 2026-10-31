// tidal-rules: UN-17
// tidal-expect: reject
// tidal-error-clang: alignof. to a function type
// tidal-error-gcc: alignof. applied to a function type
// tidal-error-edg: alignment of a function type
void f();
auto a = alignof(void());

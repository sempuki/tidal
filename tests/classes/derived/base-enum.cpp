// tidal-rules: DER-1
// tidal-expect: reject
// tidal-error-clang: base specifier must name a class
// tidal-error-gcc: expected class-name
// tidal-error-edg: not a class or struct name
enum class E {};
struct D : E {};

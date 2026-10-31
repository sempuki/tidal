// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: previously declared as scoped
// tidal-error-gcc: scoped/unscoped mismatch
// tidal-error-edg: previously declared as a different kind of enum type
enum class E : int;
enum E : int { a };

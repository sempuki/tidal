// tidal-rules: ENUM-18
// tidal-expect: reject
// tidal-error-clang: previously declared as scoped
// tidal-error-gcc: scoped/unscoped mismatch
// tidal-error-edg: different kind of enum type
enum class E : int;
enum E : int { A };

// tidal-rules: ENUM-7
// tidal-expect: reject
// tidal-error-clang: redeclared with different underlying type
// tidal-error-gcc: different underlying type
// tidal-error-edg: previously declared with a different base type
enum class F : short;
enum class F : int { A };

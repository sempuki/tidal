// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: redeclared with different underlying type
// tidal-error-gcc: different underlying type in enum
// tidal-error-edg: previously declared with a different base type
enum class E : int;
enum class E : long { a };

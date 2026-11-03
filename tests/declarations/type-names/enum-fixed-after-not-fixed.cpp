// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: previously declared with nonfixed underlying type
// tidal-error-gcc: underlying type mismatch
// tidal-error-edg: previously declared with a different base type
enum E { a };
enum E : int;

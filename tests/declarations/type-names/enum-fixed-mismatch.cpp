// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: previously declared with fixed underlying type
// tidal-error-gcc: underlying type mismatch
// tidal-error-edg: previously declared with a different base type
enum E : int;
enum E { a };

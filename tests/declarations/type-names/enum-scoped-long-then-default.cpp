// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: redeclared with different underlying type .int. \(was .long.\)
// tidal-error-gcc: different underlying type in enum
// tidal-error-edg: previously declared with a different base type
// tidal-note: A scoped enumeration declared without a type has the type int, which differs from long.
enum class E : long;
enum class E { a };

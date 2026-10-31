// tidal-rules: ENUM-16
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .Red
// tidal-error-gcc: Red. was not declared in this scope
// tidal-error-edg: identifier .Red. is undefined
enum class Color { Red };
Color c = Red;

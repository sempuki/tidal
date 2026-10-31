// tidal-rules: ENUM-18
// tidal-expect: reject
// tidal-error-clang: forward references to .enum. types
// tidal-error-gcc: use of enum .E. without previous declaration
// tidal-error-edg: forward declaration of enum type is nonstandard
enum E;

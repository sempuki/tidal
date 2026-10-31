// tidal-rules: TYN-4
// tidal-expect: reject
// tidal-error-clang: forward references to .enum. types
// tidal-error-gcc: use of enum .* without previous declaration
// tidal-error-edg: forward declaration of enum type is nonstandard
enum Undeclared* p;

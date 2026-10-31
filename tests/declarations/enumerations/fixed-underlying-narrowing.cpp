// tidal-rules: ENUM-4
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed to type .unsigned char
// tidal-error-gcc: outside the range of underlying type
// tidal-error-edg: outside the range of its underlying type
enum E : unsigned char { A = 300 };

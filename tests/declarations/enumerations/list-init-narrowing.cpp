// tidal-rules: ENUM-19
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed to type .F
// tidal-error-gcc: narrowing conversion of .300
// tidal-error-edg: invalid narrowing conversion
enum class F : unsigned char { A };
F f{300};

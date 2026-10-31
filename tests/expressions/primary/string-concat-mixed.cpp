// tidal-rules: PRIM-19
// tidal-expect: reject
// tidal-error-clang: non-standard concatenation of string literals
// tidal-error-gcc: conflicting encoding prefixes
// tidal-error-edg: different character kinds cannot be concatenated
auto s = u"a" U"b";

// tidal-rules: PRIM-33
// tidal-expect: accept
static_assert('\?' == '?' && '\"' == '"');
static_assert('\'' == "'"[0] && "\\"[1] == 0);
static_assert(sizeof("\\") == 2);
static_assert(U'\a' == 7 && U'\b' == 8 && U'\t' == 9);
static_assert(U'\n' == 10 && U'\v' == 11 && U'\f' == 12 && U'\r' == 13);
static_assert(U'\'' == 39 && U'\"' == 34 && U'\?' == 63 && U'\\' == 92);
static_assert(u8'\n' == 10 && u'\t' == 9);

// tidal-rules: PRIM-34
// tidal-expect: accept
// tidal-note: Each cell of the PRIM-34 table on the -m32 target, in the ordinary and the wide encoding.
// tidal-flags: -m32
static_assert('\'' == 0x27 && L'\'' == 0x27);
static_assert('\"' == 0x22 && L'\"' == 0x22);
static_assert('\?' == 0x3F && L'\?' == 0x3F);
static_assert('\\' == 0x5C && L'\\' == 0x5C);
static_assert('\a' == 0x07 && L'\a' == 0x07);
static_assert('\b' == 0x08 && L'\b' == 0x08);
static_assert('\f' == 0x0C && L'\f' == 0x0C);
static_assert('\n' == 0x0A && L'\n' == 0x0A);
static_assert('\r' == 0x0D && L'\r' == 0x0D);
static_assert('\t' == 0x09 && L'\t' == 0x09);
static_assert('\v' == 0x0B && L'\v' == 0x0B);

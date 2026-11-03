// tidal-rules: PRIM-30
// tidal-expect: accept
// tidal-note: Octal and hexadecimal escapes with each prefix, and single characters of the wide and UTF-16 encodings.
static_assert('\0' == 0);
static_assert(L'\x263A' == 0x263A);
static_assert(u'\x41' == 0x41 && u'\101' == 0x41);
static_assert(U'\777' == 0x1FF);
static_assert(u8'\177' == 0x7F);
static_assert(L'z' == 0x7A && u'z' == 0x7A);
static_assert(u'Ω' == 0x3A9 && U'Ω' == 0x3A9);

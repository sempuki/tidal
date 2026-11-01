// tidal-rules: PRIM-30
// tidal-expect: accept
static_assert(u8'a' == 0x61);
static_assert(u'é' == 0xE9);
static_assert(u'€' == 0x20AC);
static_assert(U'\U0001F600' == 0x1F600);
static_assert('\x41' == 0x41 && '\101' == 0x41);
static_assert(u8'\xff' == 0xFF);
static_assert('\xff' == static_cast<char>(0xFF));
static_assert('\377' == static_cast<char>(0xFF));
static_assert('A' == "A"[0] && L'A' == L"A"[0]);

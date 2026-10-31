// tidal-rules: TYP-1
// tidal-expect: accept
static_assert(sizeof(bool) == 1);
static_assert(sizeof(wchar_t) == 4);
static_assert(sizeof(char8_t) == 1 && sizeof(char16_t) == 2 && sizeof(char32_t) == 4);

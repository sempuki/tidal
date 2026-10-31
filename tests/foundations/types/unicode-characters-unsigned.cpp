// tidal-rules: TYP-3
// tidal-expect: accept
static_assert(char8_t(-1) > 0 && char16_t(-1) > 0 && char32_t(-1) > 0);
static_assert(char16_t(-1) == 0xffff);

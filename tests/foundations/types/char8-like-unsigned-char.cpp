// tidal-rules: TYP-13
// tidal-expect: accept
// tidal-note: char8_t has one byte and wraps as unsigned char does. char32_t is unsigned and holds 32 bits.
static_assert(sizeof(char8_t) == sizeof(unsigned char));
static_assert(char8_t(255) == 255 && char8_t(256) == 0);
static_assert(char8_t(-1) == static_cast<unsigned char>(-1));
static_assert(char32_t(-1) == 0xffffffff);

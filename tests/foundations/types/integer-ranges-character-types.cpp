// tidal-rules: TYP-4
// tidal-expect: accept
// tidal-note: signed char and unsigned char are integer types and also character types. Their bounds follow the TYP-4 table for a width of 8 bits, and the bounds of char16_t and char32_t follow it for 16 and 32 bits.
static_assert(static_cast<signed char>(-128) == -128);
static_assert(static_cast<signed char>(127) == 127);
static_assert(static_cast<signed char>(255) == -1);
static_assert(static_cast<unsigned char>(-1) == 255);
static_assert(static_cast<char16_t>(-1) == 65535);
static_assert(static_cast<char32_t>(-1) == 4294967295u);

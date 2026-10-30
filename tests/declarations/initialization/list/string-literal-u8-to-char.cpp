// tidal-rules: LIST-6
// tidal-expect: accept
constexpr char a[] = u8"x";
constexpr unsigned char b[] = u8"x";
static_assert(sizeof(a) == 2 && a[0] == 0x78 && b[0] == 0x78);

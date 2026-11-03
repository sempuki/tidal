// tidal-rules: SHF-2
// tidal-expect: accept
// tidal-note: Character types and short on the left, and an enumeration and bool on the right.
enum Step { Two = 2 };
static_assert(u'\x02' << Two == 8);
static_assert(static_cast<unsigned char>(3) << true == 6);
static_assert(static_cast<short>(64) >> Two == 16);
static_assert(U'\x10' >> 1 == 8);

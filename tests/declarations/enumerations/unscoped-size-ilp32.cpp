// tidal-rules: ENUM-5
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. long has 4 bytes here, so a value above the range of unsigned int needs long long or unsigned long long.
static_assert(sizeof(long) == 4);
enum Small { S = 1 };
enum Above { A = 0x80000000u };
enum Wide { W = 1LL << 40 };
enum WideNegative { N = -(1LL << 40) };
enum Top { T = 0xFFFFFFFFFFFFFFFFull };
static_assert(sizeof(Small) == 4 && sizeof(Above) == 4);
static_assert(sizeof(Wide) == 8 && sizeof(WideNegative) == 8 && sizeof(Top) == 8);

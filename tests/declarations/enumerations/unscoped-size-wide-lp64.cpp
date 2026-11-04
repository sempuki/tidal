// tidal-rules: ENUM-5
// tidal-expect: accept
// tidal-note: x86-64. A value above the range of unsigned int needs a type of 8 bytes, long or unsigned long on this target.
static_assert(sizeof(long) == 8);
enum Small { S = 1 };
enum Above { A = 0x80000000u };
enum Wide { W = 1LL << 40 };
enum WideNegative { N = -(1LL << 40) };
enum Top { T = 0xFFFFFFFFFFFFFFFFull };
static_assert(sizeof(Small) == 4 && sizeof(Above) == 4);
static_assert(sizeof(Wide) == 8 && sizeof(WideNegative) == 8 && sizeof(Top) == 8);
static_assert(alignof(Wide) == 8);

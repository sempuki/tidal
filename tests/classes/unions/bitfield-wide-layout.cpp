// tidal-rules: BF-9 BF-3
// tidal-expect: accept
// tidal-note: x86-64. A bit-field wider than its type starts at the next multiple of the alignment of the largest integer type that fits in its width, takes all its bits, and gives the class that alignment.
// int : 40 aligns as int (32 bits), and d follows the 40 bits
struct Forty { int x : 40; char d; };
static_assert(sizeof(Forty) == 8 && alignof(Forty) == 4);
// int : 64 aligns as long long
struct SixtyFour { int x : 64; };
static_assert(sizeof(SixtyFour) == 8 && alignof(SixtyFour) == 8);
// a takes byte 0, and x : 65 starts at byte 8
struct AfterChar { char a; int x : 65; };
static_assert(sizeof(AfterChar) == 24 && alignof(AfterChar) == 8);
// a takes bits of byte 0, and x : 40 starts at byte 4
struct AfterBits { char a : 4; int x : 40; };
static_assert(sizeof(AfterBits) == 12);
// c takes bits 0 to 11, e takes bits 12 to 15, and d takes byte 2
struct Shared { unsigned char c : 12; unsigned char e : 4; char d; };
static_assert(sizeof(Shared) == 3 && alignof(Shared) == 1);
// char : 16 aligns as short
struct Sixteen { char c : 16; };
static_assert(sizeof(Sixteen) == 2 && alignof(Sixteen) == 2);

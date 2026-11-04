// tidal-rules: BF-9 BF-3
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg-7.0-ilp32: reject
// tidal-error-edg-7.0-ilp32: static assertion failed
// tidal-note: -m32, where long long has alignment 4 in a class (TYP-2). Otherwise the places are those of bitfield-wide-layout.cpp.
// tidal-note: Q-0120. EDG's i686 configuration aligns long long to 8 bytes, so it gives SixtyFour and AfterChar the x86-64 alignment.
struct Forty { int x : 40; char d; };
static_assert(sizeof(Forty) == 8 && alignof(Forty) == 4);
struct SixtyFour { int x : 64; };
static_assert(sizeof(SixtyFour) == 8 && alignof(SixtyFour) == 4);
// a takes byte 0, and x : 65 starts at byte 4
struct AfterChar { char a; int x : 65; };
static_assert(sizeof(AfterChar) == 16 && alignof(AfterChar) == 4);
struct AfterBits { char a : 4; int x : 40; };
static_assert(sizeof(AfterBits) == 12);
struct Shared { unsigned char c : 12; unsigned char e : 4; char d; };
static_assert(sizeof(Shared) == 3 && alignof(Shared) == 1);
struct Sixteen { char c : 16; };
static_assert(sizeof(Sixteen) == 2 && alignof(Sixteen) == 2);

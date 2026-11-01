// tidal-rules: BF-9
// tidal-expect: accept
struct Split { unsigned a : 20; unsigned b : 20; };            // b starts the next 32-bit unit
static_assert(sizeof(Split) == 8);
struct Shorts { unsigned char a : 7; unsigned short b : 12; }; // b starts at bit 16
static_assert(sizeof(Shorts) == 4 && alignof(Shorts) == 2);
struct Mixed { char c; unsigned long long b : 30; };           // b shares the unit with c
static_assert(sizeof(Mixed) == 8 && alignof(Mixed) == 8);
struct Long { unsigned long long a : 40; unsigned long long b : 30; };  // b starts at bit 64
static_assert(sizeof(Long) == 16);
struct Cross { unsigned a : 30; unsigned long long b : 40; };  // b starts at bit 64
static_assert(sizeof(Cross) == 16);

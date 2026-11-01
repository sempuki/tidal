// tidal-rules: BF-9
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg-7.0-ilp32: reject
// tidal-error-edg-7.0-ilp32: static assertion failed
// tidal-note: Q-0120. EDG's i686 configuration aligns long long to 8 bytes, so its bit-field units of that type start at multiples of 8 bytes. GCC and Clang use multiples of 4.
struct Split { unsigned a : 20; unsigned b : 20; };
static_assert(sizeof(Split) == 8);
struct Shorts { unsigned char a : 7; unsigned short b : 12; };
static_assert(sizeof(Shorts) == 4 && alignof(Shorts) == 2);
struct Mixed { char c; unsigned long long b : 30; };
static_assert(sizeof(Mixed) == 8 && alignof(Mixed) == 4);
struct Long { unsigned long long a : 40; unsigned long long b : 30; };  // b starts at bit 40
static_assert(sizeof(Long) == 12);
struct Cross { unsigned a : 30; unsigned long long b : 40; };  // b starts at bit 32
static_assert(sizeof(Cross) == 12);

// tidal-rules: BF-9
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: x86-64. BF-9 makes the alignment implementation-defined. Clang and EDG align a bit-field of 128 bits or more as long long, 8. GCC also counts its 128-bit integer type and aligns such a bit-field to 16.
struct Wide { int x : 128; };
struct Huge { int x : 1000; };
static_assert(sizeof(Wide) == 16 && alignof(Wide) == 8);
static_assert(sizeof(Huge) == 128 && alignof(Huge) == 8);

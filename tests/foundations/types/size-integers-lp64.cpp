// tidal-rules: TYP-1
// tidal-expect: accept
static_assert(sizeof(char) == 1 && sizeof(signed char) == 1 && sizeof(unsigned char) == 1);
static_assert(sizeof(short) == 2 && sizeof(unsigned short) == 2);
static_assert(sizeof(int) == 4 && sizeof(unsigned) == 4);
static_assert(sizeof(long) == 8 && sizeof(unsigned long) == 8);
static_assert(sizeof(long long) == 8 && sizeof(unsigned long long) == 8);

// tidal-rules: CAST-16
// tidal-expect: accept
// tidal-flags: -m32
using size = decltype(sizeof 0);
static_assert(sizeof(void*) == 4);
static_assert(sizeof(size) == sizeof(void*) && sizeof(unsigned long) == sizeof(void*));
static_assert(sizeof(unsigned) == sizeof(void*));

// tidal-rules: OPR-10
// tidal-expect: reject
using Size = decltype(sizeof 0);
int operator""_t(const char*, Size, int);

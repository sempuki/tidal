// tidal-rules: OPR-8
// tidal-expect: reject
using Size = decltype(sizeof 0);
int operator""_t(const char*, Size, int);

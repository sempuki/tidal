// tidal-rules: OPR-10
// tidal-expect: reject
// tidal-error-clang: literal operator must have one or two parameters
// tidal-error-gcc: has invalid argument list
// tidal-error-edg: too many parameters for this literal operator
using Size = decltype(sizeof 0);
int operator""_t(const char*, Size, int);

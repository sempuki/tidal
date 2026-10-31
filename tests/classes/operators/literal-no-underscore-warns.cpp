// tidal-rules: OPR-9
// tidal-expect: reject
// tidal-error-clang: not starting with ._. are reserved
// tidal-error-gcc: suffixes not preceded by ._. are reserved
// tidal-flags: -Werror
int operator""km(unsigned long long);

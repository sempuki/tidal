// tidal-rules: VOL-10
// tidal-expect: reject
// tidal-error-clang: drops .volatile. qualifier
// tidal-error-gcc: to .volatile int. discards qualifiers
// tidal-error-edg: qualifiers dropped in binding reference
volatile int v;
int& r = v;

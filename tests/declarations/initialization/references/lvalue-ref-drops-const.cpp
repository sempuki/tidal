// tidal-rules: REF-8
// tidal-expect: reject
// tidal-error-clang: drops .const. qualifier
// tidal-error-gcc: discards qualifiers
// tidal-error-edg: qualifiers dropped in binding reference
const int c = 1;
int& r = c;

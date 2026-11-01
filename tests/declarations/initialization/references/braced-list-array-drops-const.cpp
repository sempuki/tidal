// tidal-rules: REF-6 REF-8
// tidal-expect: reject
// tidal-error-clang: drops .const. qualifier
// tidal-error-gcc: discards qualifiers
// tidal-error-edg: qualifiers dropped in binding reference
const int fixed[2] = {1, 2};
int (&loose)[2]{fixed};

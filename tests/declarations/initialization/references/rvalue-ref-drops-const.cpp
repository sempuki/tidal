// tidal-rules: REF-10
// tidal-expect: reject
// tidal-error-clang: drops .const. qualifier
// tidal-error-gcc: discards qualifiers
// tidal-error-edg: qualifiers dropped in binding reference
// tidal-note: Gap review 2026-10-31.
const int c = 1;
int&& r = static_cast<const int&&>(c);

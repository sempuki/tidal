// tidal-rules: REF-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
const int c = 1;
int&& r = static_cast<const int&&>(c);

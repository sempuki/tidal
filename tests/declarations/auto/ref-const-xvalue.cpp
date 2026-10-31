// tidal-rules: AUTO-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
const int c = 1;
auto& r = static_cast<const int&&>(c);

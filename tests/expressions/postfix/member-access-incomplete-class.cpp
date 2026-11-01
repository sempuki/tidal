// tidal-rules: PFX-11 CLS-22
// tidal-expect: reject
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-clang: member access into incomplete type
// tidal-error-edg: incomplete type
// tidal-note: Gauge is declared and never defined, so the member access has no class to search.
struct Gauge;
int read(Gauge& g) { return g.v; }

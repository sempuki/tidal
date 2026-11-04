// tidal-rules: VC-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
int g = 0;
struct R { int& r; };
R make() { return {g}; }
int& x = make().r;

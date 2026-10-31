// tidal-rules: CLS-12
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
void g() { struct L { friend void h() {} }; }

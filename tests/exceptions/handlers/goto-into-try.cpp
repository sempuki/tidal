// tidal-rules: TRY-7
// tidal-expect: reject
void g() { goto inside; try { inside: ; } catch (...) { } }

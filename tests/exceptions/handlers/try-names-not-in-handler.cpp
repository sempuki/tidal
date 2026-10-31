// tidal-rules: TRY-14
// tidal-expect: reject
void g() { try { int inside = 1; } catch (...) { int w = inside; } }

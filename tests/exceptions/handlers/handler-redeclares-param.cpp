// tidal-rules: TRY-14
// tidal-expect: reject
void g() { try { } catch (int v) { int v = 2; } }

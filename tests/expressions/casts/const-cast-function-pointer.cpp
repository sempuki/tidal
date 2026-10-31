// tidal-rules: CAST-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
void g();
void (*p)() = const_cast<void (*)()>(&g);

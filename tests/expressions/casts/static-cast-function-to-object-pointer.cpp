// tidal-rules: CAST-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
void g();
void* p = static_cast<void*>(&g);

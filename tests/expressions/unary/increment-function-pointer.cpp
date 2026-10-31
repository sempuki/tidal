// tidal-rules: UN-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
void g();
void f() { void (*p)() = g; ++p; }

// tidal-rules: LOOP-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f() { for (static_assert(true); ;) break; }

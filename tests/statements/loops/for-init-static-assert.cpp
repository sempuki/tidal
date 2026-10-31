// tidal-rules: LOOP-7
// tidal-expect: reject
// tidal-error-gcc: expected primary-expression before .static_assert
// tidal-error-edg: this declaration is not valid here
// tidal-note: Gap review 2026-10-31 (third).
void f() { for (static_assert(true); ;) break; }

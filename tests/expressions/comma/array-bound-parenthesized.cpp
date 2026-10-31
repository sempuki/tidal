// tidal-rules: COMMA-8
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
int a[(1, 2)];
static_assert(sizeof(a) == 2 * sizeof(int));

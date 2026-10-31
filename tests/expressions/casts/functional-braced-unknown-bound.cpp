// tidal-rules: CAST-18
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
using A = int[];
static_assert(sizeof(A{1, 2, 3}) == 3 * sizeof(int));

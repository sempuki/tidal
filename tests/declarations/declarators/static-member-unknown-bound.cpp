// tidal-rules: DCL-5
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct S { static int table[]; };
int S::table[3] = {1, 2, 3};
static_assert(sizeof(S::table) == 3 * sizeof(int));

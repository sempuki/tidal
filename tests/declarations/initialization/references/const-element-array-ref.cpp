// tidal-rules: REF-1
// tidal-expect: accept
// tidal-note: const on the elements counts as const on the array, so each reference binds directly to cells. known refers to the same object as cells.
int cells[2];
const int (&known)[2] = cells;
const int (&unknown)[] = cells;
volatile int (&touched)[2] = cells;
static_assert(&known[0] == &cells[0]);
static_assert(&known == &cells);

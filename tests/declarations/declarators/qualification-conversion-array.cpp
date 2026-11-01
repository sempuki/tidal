// tidal-rules: CV-7
// tidal-expect: accept
// tidal-note: const on the elements of an array counts as const on the array. A pointer to an array of known bound also converts to a pointer to an array of unknown bound.
int grid[3];
int (*row)[3] = &grid;
const int (*fixed)[3] = row;
const int (*open)[] = row;
int (*plain)[] = row;
int (**rows)[3] = &row;
const int (* const *view)[] = rows;
int matrix[2][3];
const int (*part)[][3] = &matrix;

// tidal-rules: NAM-41
// tidal-expect: reject
// tidal-error-clang: no type named .cell. in .grid
// tidal-error-gcc: cell. in .struct grid. does not name a type
// tidal-error-edg: member .grid::cell. is not a type name
// tidal-note: grid::cell finds the data member, so the qualified name alone cannot name the nested class.
struct grid { struct cell { int v; }; int cell = 0; };
grid::cell first;

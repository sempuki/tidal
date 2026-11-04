// tidal-rules: DEFARG-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: makes this constructor a move constructor
// tidal-error-edg: added default arguments cannot result
// tidal-note: Q-0370. Clang rejects a default argument outside the class that turns a constructor into a move constructor. GCC accepts it.
struct Cell {
  Cell();
  Cell(Cell&&, int);
};
Cell::Cell(Cell&&, int = 0) {}

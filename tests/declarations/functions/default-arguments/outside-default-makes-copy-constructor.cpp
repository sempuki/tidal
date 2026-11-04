// tidal-rules: DEFARG-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: makes this constructor a copy constructor
// tidal-error-edg: added default arguments cannot result in declaring a default or copy
// tidal-note: Q-0370. Clang rejects a default argument outside the class that turns a constructor into a copy constructor. GCC accepts it.
struct Cell {
  Cell();
  Cell(const Cell&, int);
};
Cell::Cell(const Cell&, int = 0) {}

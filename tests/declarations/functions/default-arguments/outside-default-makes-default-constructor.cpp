// tidal-rules: DEFARG-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: makes this constructor a default constructor
// tidal-error-edg: added default arguments cannot result in declaring a default
// tidal-note: Q-0370. Clang rejects a default argument outside the class that turns a constructor into a default constructor. GCC accepts it.
struct Cell {
  int v;
  Cell(int x);
};
Cell::Cell(int x = 5) : v(x) {}

// tidal-rules: DEFARG-10
// tidal-expect: accept
// tidal-note: The default argument of a member function is checked as if at the end of the class, so it can use a static member and a nested class that the class declares later.
struct Grid {
  int cells(int n = side * side) { return n; }
  int bytes(int n = sizeof(Cell)) { return n; }
  static const int side = 4;
  struct Cell { char data[5]; };
};
static_assert(sizeof(Grid::Cell) == 5);

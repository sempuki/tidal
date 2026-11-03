// tidal-rules: NAM-39
// tidal-expect: accept
// tidal-note: A class has a nested class and a data member with one name, a nested class and a member function with another, and a nested class and an enumerator with a third. In the last pair, the data member comes first.
struct table {
  struct row { int id; };
  int row = 0;
  struct cell { int v; };
  int cell() const;
  int column = 1;
  struct column { int w; };
  struct key { int k; };
  enum { key = 2 };
};

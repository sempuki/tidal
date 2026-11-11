// tidal-rules: OVL-7 OVL-25
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction assignment_beside_converting_assignment.cpp (from bits_first_try.cpp), a second form of exercise row P3-10. Both operator= functions are candidates (OVL-7). The copy assignment takes y by an exact match, and operator=(int) needs a user-defined conversion, so the copy assignment is better (OVL-25).
struct Cell {
  int* slot;
  Cell& operator=(int v) { *slot = v * 10; return *this; }
  Cell& operator=(const Cell& other) { *slot = *other.slot; return *this; }
  operator int() const { return *slot; }
};
int main() {
  int a = 0, b = 4;
  Cell x{&a}, y{&b};
  x = y;
  return a == 4 ? 0 : 1;
}

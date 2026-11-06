// tidal-rules: DCL-5 INIT-5
// tidal-expect: run
// tidal-note: Roadmap round 9 (P1-1), P1 reduction shapes-unknown-bound-converting.cpp. An array of class type with no bound takes its bound from the braced list, and each element is made through a converting constructor.
struct Cell {
  int v;
  Cell(int x) : v(x) {}
};
int main() {
  Cell row[] = {4, 5, 6};
  static_assert(sizeof row == 3 * sizeof(Cell));
  return row[2].v == 6 ? 0 : 1;
}

// tidal-rules: DCMP-1
// tidal-expect: run
// tidal-note: Write suite W19, finding W19-1 (reports/clean-room/W19/report.md). A defaulted == compares a member that is an array of arrays element by element, down to the int elements.
struct M {
  int c[2][2];
  bool operator==(const M&) const = default;
};
int main() {
  M x{{{1, 2}, {3, 4}}}, y{{{1, 2}, {3, 4}}}, z{{{1, 2}, {3, 5}}}, w{{{1, 9}, {3, 4}}};
  return x == y && !(x == z) && !(x == w) ? 0 : 1;
}

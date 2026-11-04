// tidal-rules: DEFARG-3
// tidal-expect: run
struct Grid {
  int width(int w = standard) { return w; }   // standard is declared later
  static const int standard = 3;
};
int main() { return Grid{}.width() == 3 ? 0 : 1; }

// tidal-rules: LNK-6
// tidal-expect: run
// tidal-sources: inline-different-bodies.tu.cpp
inline int which() { return 1; }
int which_from_other_unit();
int main() {
  int here = which();
  int there = which_from_other_unit();
  bool each_valid = (here == 1 || here == 2) && (there == 1 || there == 2);
  return each_valid ? 0 : 1;
}

// tidal-rules: CMP-8
// tidal-expect: run
// tidal-note: CMP-8 lets a < b and a > b give any pair of results, so the program checks only that each comparison runs and gives true or false.
int a, b;
int main() {
  bool lt = &a < &b, gt = &a > &b;
  int count = lt + gt;
  return count >= 0 && count <= 2 ? 0 : 1;
}

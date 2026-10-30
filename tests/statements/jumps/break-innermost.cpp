// tidal-rules: JMP-1
// tidal-expect: run
int main() {
  int outer = 0;
  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 3; ++j) { if (j == 1) break; }
    ++outer;
  }
  return outer == 3 ? 0 : 1;
}

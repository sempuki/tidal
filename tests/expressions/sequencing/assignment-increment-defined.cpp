// tidal-rules: SEQ-2
// tidal-expect: run
int main() {
  int i = 1;
  i = i++ + 1;
  int j = 1;
  j = ++j + 1;
  return i == 2 && j == 3 ? 0 : 1;
}

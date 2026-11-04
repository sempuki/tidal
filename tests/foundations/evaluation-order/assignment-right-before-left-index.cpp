// tidal-rules: SEQ-2
// tidal-expect: run
int main() {
  int cells[3] = {9, 9, 9};
  int i = 0;
  cells[i++] = i;
  return cells[0] == 0 && cells[1] == 9 && i == 1 ? 0 : 1;
}

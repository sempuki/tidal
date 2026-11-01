// tidal-rules: JMP-1
// tidal-expect: run
// tidal-note: A range-based for is a loop, so break ends it.
int main() {
  int values[] = {1, 2, 3, 4};
  int sum = 0;
  for (int v : values) {
    if (v == 3) break;
    sum += v;
  }
  return sum == 3 ? 0 : 1;
}

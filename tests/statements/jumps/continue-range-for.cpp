// tidal-rules: JMP-2
// tidal-expect: run
// tidal-note: continue in a range-based for goes to the end of the body. The loop then advances to the next element, and the body runs once for each element.
int main() {
  int values[] = {1, 2, 3, 4};
  int sum = 0, visits = 0;
  for (int v : values) {
    ++visits;
    if (v % 2 == 0) continue;
    sum += v;
  }
  return sum == 4 && visits == 4 ? 0 : 1;
}

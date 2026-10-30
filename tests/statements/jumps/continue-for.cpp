// tidal-rules: JMP-2
// tidal-expect: run
int main() {
  int sum = 0, steps = 0;
  for (int i = 0; i < 4; ++i, ++steps) { if (i % 2) continue; sum += i; }
  return sum == 2 && steps == 4 ? 0 : 1;
}

// tidal-rules: JMP-2
// tidal-expect: run
int main() {
  int checks = 0, i = 0;
  while (++checks, i < 3) { ++i; continue; }
  return checks == 4 ? 0 : 1;
}

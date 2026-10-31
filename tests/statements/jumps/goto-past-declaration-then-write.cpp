// tidal-rules: JMP-16
// tidal-expect: run
int main() {
  int r = 0;
  goto later;
  {
  }
  int x;
later:
  x = 7;
  r = x;
  return r == 7 ? 0 : 1;
}

// tidal-rules: SEQ-1
// tidal-expect: run
int i = 0;
int pair(int x, int y) { return x * 10 + y; }
int main() {
  int r = pair(i++, i++);
  return (r == 1 || r == 10) && i == 2 ? 0 : 1;
}
